//! LumenLan — janela nativa (Tauri v2). O servidor HTTP+WebSocket que atende
//! a LAN entra na Fase 1, spawnado no hook `setup()`.

mod config;
mod files;
mod protocol;
mod server;

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

    tauri::Builder::default()
        .setup(move |_app| {
            tracing::info!("LumenLan iniciado");
            // Servidor HTTP+WebSocket em task dedicada no runtime do Tauri.
            tauri::async_runtime::spawn(server::run(shutdown_rx.clone()));
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
