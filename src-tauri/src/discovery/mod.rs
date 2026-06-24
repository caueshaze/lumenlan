//! Autodescoberta via mDNS: anuncia o servico na LAN para que o host seja
//! encontravel como `lumenlan.local` sem digitar IP manualmente.
//!
//! Navegadores nao resolvem servicos `_lumenlan._tcp` arbitrarios, mas o
//! registro do host (`lumenlan.local`) permite, em redes compativeis, acessar
//! `http://lumenlan.local:PORTA`. O caminho garantido continua sendo o QR code
//! exibido na janela nativa.

use std::sync::OnceLock;

use mdns_sd::{ServiceDaemon, ServiceInfo};

use crate::config;

/// Mantem o daemon vivo pela duracao do processo (se dropado, para de anunciar).
static DAEMON: OnceLock<ServiceDaemon> = OnceLock::new();

/// Tenta anunciar o servico. Falhas (rede sem multicast etc.) sao apenas logadas.
pub fn advertise() {
    if let Err(e) = try_advertise() {
        tracing::warn!("mDNS indisponivel: {e}");
    }
}

fn try_advertise() -> Result<(), Box<dyn std::error::Error>> {
    let daemon = ServiceDaemon::new()?;
    let ip = config::local_ip();
    let port = config::SERVER_PORT;

    let service = ServiceInfo::new(
        "_lumenlan._tcp.local.",
        "LumenLan",
        "lumenlan.local.",
        ip.as_str(),
        port,
        &[("path", "/")][..],
    )?;

    daemon.register(service)?;
    tracing::info!("mDNS anunciado: lumenlan.local -> {ip}:{port}");
    let _ = DAEMON.set(daemon);
    Ok(())
}
