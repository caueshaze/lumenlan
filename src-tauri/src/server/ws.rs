//! Handler de cada conexao WebSocket.
//!
//! Um unico laco com `select!` cuida das duas direcoes ao mesmo tempo:
//! mensagens vindas do barramento (outros clientes) vao para o socket, e
//! mensagens do socket sao publicadas no barramento. Sem split -> sem deps extras.

use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        ConnectInfo, Query, State,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use serde::Deserialize;
use tokio::sync::broadcast::error::RecvError;

use crate::files::{self, Receiver};
use crate::protocol::{now_ms, ClientMsg, ServerMsg};

use super::hub::{Hub, Payload};
use super::AppState;

#[derive(Deserialize)]
pub struct WsAuth {
    token: Option<String>,
}

/// Rota `/ws`: exige o PIN de sala antes do upgrade. IPs com muitas tentativas
/// erradas ficam em cooldown (429); PIN incorreto recebe 401.
pub async fn upgrade(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(auth): Query<WsAuth>,
) -> Response {
    let ip = addr.ip();
    if !state.throttle.allowed(ip) {
        return (StatusCode::TOO_MANY_REQUESTS, "muitas tentativas, tente em instantes").into_response();
    }
    if auth.token.as_deref() != Some(state.token.as_ref()) {
        state.throttle.record_fail(ip);
        return (StatusCode::UNAUTHORIZED, "PIN de sala invalido").into_response();
    }
    state.throttle.record_ok(ip);
    let hub = state.hub.clone();
    ws.on_upgrade(move |socket| async move { Conn::new(&hub).run(socket, hub.clone()).await })
}

/// Estado de uma conexao. `incoming` guarda a transferencia de imagem em curso
/// (uma por vez por conexao, pois os frames chegam ordenados).
struct Conn {
    id: u64,
    name: String,
    incoming: Option<(String, Receiver)>,
}

impl Conn {
    fn new(hub: &Hub) -> Self {
        Self {
            id: hub.next_id(),
            name: String::from("anon"),
            incoming: None,
        }
    }

    async fn run(mut self, mut socket: WebSocket, hub: Hub) {
        hub.join();
        let mut rx = hub.subscribe();

        loop {
            tokio::select! {
                // Saida: o que outros clientes publicaram -> envia a este socket.
                received = rx.recv() => match received {
                    Ok(env) => {
                        if env.origin == self.id {
                            continue; // nao ecoa para a propria origem
                        }
                        let msg = match env.payload {
                            Payload::Json(s) => Message::Text(s.into()),
                            Payload::Binary(b) => Message::Binary(b),
                        };
                        if socket.send(msg).await.is_err() {
                            break;
                        }
                    }
                    Err(RecvError::Lagged(n)) => {
                        tracing::warn!("conexao {} atrasou {n} mensagens", self.id);
                        continue;
                    }
                    Err(RecvError::Closed) => break,
                },

                // Entrada: o que este cliente enviou.
                incoming = socket.recv() => match incoming {
                    Some(Ok(Message::Text(text))) => self.on_text(&hub, text.as_str()).await,
                    Some(Ok(Message::Binary(bin))) => self.on_binary(&hub, bin).await,
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(_)) => {} // ping/pong
                    Some(Err(e)) => {
                        tracing::debug!("erro de leitura na conexao {}: {e}", self.id);
                        break;
                    }
                },
            }
        }

        hub.leave();
        if self.name != "anon" {
            hub.publish_server(&ServerMsg::System {
                body: format!("{} saiu", self.name),
            });
        }
    }

    /// Interpreta um frame de texto (JSON) vindo do cliente.
    async fn on_text(&mut self, hub: &Hub, raw: &str) {
        let Ok(msg) = serde_json::from_str::<ClientMsg>(raw) else {
            tracing::debug!("frame ignorado (json invalido)");
            return;
        };

        match msg {
            ClientMsg::Hello { name } => {
                self.name = sanitize_name(&name);
                hub.publish_server(&ServerMsg::System {
                    body: format!("{} entrou", self.name),
                });
            }
            ClientMsg::Text { body } => {
                let body = body.trim();
                if body.is_empty() {
                    return;
                }
                hub.publish(
                    self.id,
                    Payload::Json(
                        ServerMsg::Text {
                            from: self.name.clone(),
                            body: body.to_string(),
                            ts: now_ms(),
                        }
                        .to_json(),
                    ),
                );
            }
            ClientMsg::FileStart {
                id,
                name,
                mime,
                size,
            } => self.start_file(hub, id, name, mime, size).await,
            ClientMsg::FileEnd { id } => self.end_file(hub, id).await,
        }
    }

    /// Chunk binario cru: grava no disco e repassa enquadrado aos demais.
    async fn on_binary(&mut self, hub: &Hub, data: Bytes) {
        let Some((id, receiver)) = self.incoming.as_mut() else {
            return; // chunk sem FileStart correspondente
        };
        if let Err(e) = receiver.write(&data).await {
            tracing::error!("transferencia abortada: {e}");
            self.incoming = None;
            self.reject(hub, "falha ao gravar arquivo".into());
            return;
        }
        hub.publish(self.id, Payload::Binary(files::frame_chunk(id, &data)));
    }

    /// Recusa a transferencia atual e avisa o cliente.
    fn reject(&self, hub: &Hub, reason: String) {
        hub.publish_server(&ServerMsg::System {
            body: format!("imagem recusada: {reason}"),
        });
    }

    async fn start_file(&mut self, hub: &Hub, id: String, name: String, mime: String, size: u64) {
        match Receiver::create(&name).await {
            Ok(receiver) => {
                self.incoming = Some((id.clone(), receiver));
                hub.publish_server(&ServerMsg::FileStart {
                    from: self.name.clone(),
                    id,
                    name,
                    mime,
                    size,
                });
            }
            Err(e) => {
                tracing::error!("nao foi possivel criar arquivo: {e}");
                hub.publish_server(&ServerMsg::System {
                    body: "falha ao receber imagem".into(),
                });
            }
        }
    }

    async fn end_file(&mut self, hub: &Hub, id: String) {
        if let Some((active_id, receiver)) = self.incoming.take() {
            if active_id == id {
                if let Err(e) = receiver.finish().await {
                    tracing::error!("falha ao finalizar arquivo: {e}");
                }
            }
        }
        hub.publish_server(&ServerMsg::FileEnd { id });
    }
}

/// Limita o nome a algo curto e sem quebras de linha.
fn sanitize_name(raw: &str) -> String {
    let cleaned: String = raw.trim().chars().filter(|c| !c.is_control()).take(32).collect();
    if cleaned.is_empty() {
        "anon".to_string()
    } else {
        cleaned
    }
}
