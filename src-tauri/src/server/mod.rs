//! Servidor HTTP + WebSocket embutido. Liga em 0.0.0.0:PORTA para atender tanto
//! a janela nativa (via localhost) quanto os celulares na LAN.

mod hub;
mod router;
mod static_files;
mod ws;

use std::net::SocketAddr;
use std::sync::Arc;

use crate::config;

use hub::Hub;

use crate::auth::Throttle;

/// Estado compartilhado pelas rotas: broadcast + PIN de sala + anti-brute-force.
#[derive(Clone)]
pub struct AppState {
    pub hub: Hub,
    pub token: Arc<str>,
    pub throttle: Throttle,
}

/// Sobe o servidor. Roda ate o `shutdown` sinalizar (fechamento da janela) ou
/// o processo encerrar; chamado em uma task dedicada a partir do `setup()`.
pub async fn run(mut shutdown: tokio::sync::watch::Receiver<bool>, token: Arc<str>) {
    let addr: SocketAddr = format!("{}:{}", config::BIND_ADDR, config::SERVER_PORT)
        .parse()
        .expect("endereco de bind invalido");

    let token_hint = token.clone();
    let app = router::build(AppState {
        hub: Hub::new(),
        token,
        throttle: Throttle::default(),
    });

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("falha ao ligar o servidor em {addr}: {e}");
            return;
        }
    };

    tracing::info!(
        "servidor no ar — abra no celular: http://{}:{}/?t={}",
        config::local_ip(),
        config::SERVER_PORT,
        token_hint
    );

    // `into_make_service_with_connect_info` expoe o IP do cliente aos handlers
    // (necessario para o limitador de tentativas por IP).
    let service = app.into_make_service_with_connect_info::<SocketAddr>();
    let graceful = axum::serve(listener, service).with_graceful_shutdown(async move {
        let _ = shutdown.changed().await;
        tracing::info!("desligando servidor (graceful)");
    });

    if let Err(e) = graceful.await {
        tracing::error!("servidor encerrou com erro: {e}");
    }
}
