//! Montagem das rotas HTTP do servidor.

use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::config;

use super::{hub::Hub, static_files, ws};

/// Resposta de `/health`: usada pela UI para descobrir o endereco que os
/// celulares devem acessar e checar se o servidor esta no ar.
#[derive(Serialize)]
struct Health {
    status: &'static str,
    ip: String,
    port: u16,
}

async fn health() -> Json<Health> {
    Json(Health {
        status: "ok",
        ip: config::local_ip(),
        port: config::SERVER_PORT,
    })
}

/// Constroi o roteador: `/health`, `/ws` e fallback servindo o frontend.
pub fn build(hub: Hub) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ws", get(ws::upgrade))
        .with_state(hub)
        .fallback(static_files::serve)
}
