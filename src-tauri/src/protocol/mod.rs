//! Protocolo de mensagens da rede (contrato serde).

mod message;

pub use message::{now_ms, ClientMsg, ServerMsg};
