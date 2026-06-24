//! Serve os assets do frontend Svelte embutidos no binario (self-contained).
//!
//! O build do Vite (`frontend/dist`) e compilado para dentro do executavel via
//! `rust-embed`, entao os celulares recebem a UI direto do binario, sem arquivos
//! soltos no disco.

use axum::{
    body::Body,
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../frontend/dist"]
struct Assets;

/// Handler de fallback: resolve o caminho da URL para um asset embutido.
/// Caminhos desconhecidos caem em `index.html` (comportamento SPA).
pub async fn serve(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    match Assets::get(path) {
        Some(file) => asset_response(path, file.data.into_owned()),
        None => match Assets::get("index.html") {
            Some(index) => asset_response("index.html", index.data.into_owned()),
            None => (StatusCode::NOT_FOUND, "frontend nao embutido").into_response(),
        },
    }
}

fn asset_response(path: &str, bytes: Vec<u8>) -> Response {
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime.as_ref())
        .body(Body::from(bytes))
        .unwrap()
}
