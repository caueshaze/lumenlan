//! Servidor HTTP + WebSocket embutido. Liga em 0.0.0.0:PORTA para atender tanto
//! a janela nativa (via localhost) quanto os celulares na LAN.

mod hub;
mod router;
mod static_files;
mod ws;

use std::net::SocketAddr;

use crate::config;

/// Sobe o servidor. Roda ate o `shutdown` sinalizar (fechamento da janela) ou
/// o processo encerrar; chamado em uma task dedicada a partir do `setup()`.
pub async fn run(mut shutdown: tokio::sync::watch::Receiver<bool>) {
    let addr: SocketAddr = format!("{}:{}", config::BIND_ADDR, config::SERVER_PORT)
        .parse()
        .expect("endereco de bind invalido");

    let app = router::build(hub::Hub::new());

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("falha ao ligar o servidor em {addr}: {e}");
            return;
        }
    };

    tracing::info!(
        "servidor no ar — celulares acessam http://{}:{}",
        config::local_ip(),
        config::SERVER_PORT
    );

    let graceful = axum::serve(listener, app).with_graceful_shutdown(async move {
        let _ = shutdown.changed().await;
        tracing::info!("desligando servidor (graceful)");
    });

    if let Err(e) = graceful.await {
        tracing::error!("servidor encerrou com erro: {e}");
    }
}
