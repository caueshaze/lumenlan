//! Contrato serde das mensagens trafegadas via WebSocket.
//!
//! Controle e texto viajam como frames de TEXTO (JSON). Os bytes de imagem
//! (Fase 3) viajam como frames BINARIOS crus, precedidos por `FileStart` e
//! encerrados por `FileEnd` — assim nao pagamos o custo de base64 na RAM.

use serde::{Deserialize, Serialize};

/// Mensagens enviadas pelo cliente (navegador/janela) ao servidor.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMsg {
    /// Identificacao ao conectar.
    Hello { name: String },
    /// Mensagem de texto do chat.
    Text { body: String },
    /// Inicio de transferencia de imagem (seguido de frames binarios).
    FileStart {
        id: String,
        name: String,
        mime: String,
        size: u64,
    },
    /// Fim da transferencia identificada por `id`.
    FileEnd { id: String },
}

/// Mensagens enviadas pelo servidor aos clientes.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMsg {
    /// Texto de chat retransmitido, com autor e timestamp (ms).
    Text { from: String, body: String, ts: u64 },
    /// Quantidade atual de clientes conectados.
    Presence { count: usize },
    /// Aviso do sistema (entrou/saiu, erros amigaveis).
    System { body: String },
    /// Inicio de imagem repassada de outro cliente.
    FileStart {
        from: String,
        id: String,
        name: String,
        mime: String,
        size: u64,
    },
    /// Fim da imagem repassada.
    FileEnd { id: String },
}

impl ServerMsg {
    /// Serializa para JSON; em caso raro de falha, devolve um `System` de erro.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| r#"{"type":"system","body":"erro de serializacao"}"#.to_string())
    }
}

/// Timestamp atual em milissegundos desde a epoch.
pub fn now_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
