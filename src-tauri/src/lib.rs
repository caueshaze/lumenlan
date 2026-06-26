//! LumenLan — janela nativa (Tauri v2). O servidor HTTP+WebSocket que atende
//! a LAN entra na Fase 1, spawnado no hook `setup()`.

mod auth;
mod config;
mod discovery;
mod files;
mod protocol;
mod server;

use std::sync::Arc;

/// Comando IPC: entrega o PIN de sala apenas a janela nativa (que roda Tauri).
/// Os demais recebem o PIN pelo QR code ou digitando-o, nunca por este caminho.
#[tauri::command]
fn app_token(token: tauri::State<'_, Arc<str>>) -> String {
    token.to_string()
}

/// IP e porta do servidor para a janela nativa montar a URL/QR. Necessario
/// porque a webview nao pode fazer `fetch` cross-origin em `/health`.
#[derive(serde::Serialize)]
struct ServerInfo {
    ip: String,
    port: u16,
}

#[tauri::command]
fn server_info() -> ServerInfo {
    ServerInfo {
        ip: config::local_ip(),
        port: config::SERVER_PORT,
    }
}

/// Ponto de entrada compartilhado entre o binario desktop e os alvos mobile.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "lumenlan_lib=info,tower_http=info".into()),
        )
        .init();

    // Canal de desligamento: sinalizado quando a janela e fechada, permitindo
    // ao servidor encerrar de forma graciosa.
    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);

    // PIN de sala desta sessao (novo a cada execucao).
    let token = auth::generate_pin();
    let server_token = token.clone();

    tauri::Builder::default()
        .manage(token)
        .invoke_handler(tauri::generate_handler![app_token, server_info])
        .setup(move |_app| {
            tracing::info!("LumenLan iniciado");
            // Servidor HTTP+WebSocket em task dedicada no runtime do Tauri.
            tauri::async_runtime::spawn(server::run(shutdown_rx.clone(), server_token.clone()));
            // Anuncia o host na LAN via mDNS (best-effort).
            discovery::advertise();
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("erro ao iniciar a aplicacao Tauri")
        .run(move |_app, event| {
            if let tauri::RunEvent::ExitRequested { .. } = event {
                let _ = shutdown_tx.send(true);
            }
        });
}
