//! Constantes e caminhos compartilhados pelo app.

use std::net::UdpSocket;
use std::path::PathBuf;

/// Porta TCP onde o servidor HTTP + WebSocket escuta (ligado em 0.0.0.0).
pub const SERVER_PORT: u16 = 8787;

/// Endereco de bind do servidor: todas as interfaces, para alcancar a LAN.
pub const BIND_ADDR: &str = "0.0.0.0";

/// Pasta onde imagens recebidas sao salvas automaticamente.
pub const DOWNLOAD_DIR: &str = "lumenlan_received";

/// Tamanho maximo aceito por imagem (50 MiB).
pub const MAX_FILE_BYTES: u64 = 50 * 1024 * 1024;

/// Aceita apenas imagens. Usa o mime quando presente; senao infere pela extensao.
pub fn is_allowed_image(mime: &str, name: &str) -> bool {
    if mime.starts_with("image/") {
        return true;
    }
    if !mime.is_empty() {
        return false;
    }
    let lower = name.to_ascii_lowercase();
    [".png", ".jpg", ".jpeg", ".gif", ".webp", ".bmp", ".svg", ".avif", ".heic"]
        .iter()
        .any(|ext| lower.ends_with(ext))
}

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
