//! Ferramenta de dev: navega por servicos `_lumenlan._tcp` na LAN por ~4s.
//! Uso: `cargo run --example mdns_browse`

use std::time::Duration;

use mdns_sd::{ServiceDaemon, ServiceEvent};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mdns = ServiceDaemon::new()?;
    let receiver = mdns.browse("_lumenlan._tcp.local.")?;
    let deadline = std::time::Instant::now() + Duration::from_secs(4);

    println!("procurando _lumenlan._tcp por 4s...");
    while std::time::Instant::now() < deadline {
        if let Ok(event) = receiver.recv_timeout(Duration::from_millis(500)) {
            if let ServiceEvent::ServiceResolved(info) = event {
                println!(
                    "ENCONTRADO: {} host={} addrs={:?} port={}",
                    info.get_fullname(),
                    info.get_hostname(),
                    info.get_addresses(),
                    info.get_port()
                );
            }
        }
    }
    Ok(())
}
