//! Constantes e caminhos compartilhados pelo app.

use std::net::UdpSocket;
use std::path::PathBuf;

/// Porta TCP onde o servidor HTTP + WebSocket escuta (ligado em 0.0.0.0).
pub const SERVER_PORT: u16 = 8787;

/// Endereco de bind do servidor: todas as interfaces, para alcancar a LAN.
pub const BIND_ADDR: &str = "0.0.0.0";

/// Pasta onde arquivos recebidos sao salvos automaticamente.
pub const DOWNLOAD_DIR: &str = "lumenlan_received";

/// Resolve o diretorio de downloads relativo ao CWD do processo.
pub fn download_dir() -> PathBuf {
    PathBuf::from(DOWNLOAD_DIR)
}

/// Descobre o IP da maquina na LAN sem trafego real: "conecta" um socket UDP
/// a um destino externo so para o SO escolher a interface de saida e revelar
/// o endereco local. Cai para 127.0.0.1 se nao houver rota.
pub fn local_ip() -> String {
    UdpSocket::bind("0.0.0.0:0")
        .and_then(|sock| {
            sock.connect("8.8.8.8:80")?;
            Ok(sock.local_addr()?.ip().to_string())
        })
        .unwrap_or_else(|_| "127.0.0.1".to_string())
}
