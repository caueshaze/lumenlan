//! Barramento de broadcast 1->N e contagem de clientes conectados.
//!
//! Cada conexao recebe um `id` unico e assina o canal. Ao publicar, a origem
//! e marcada para que o proprio remetente nao receba de volta (o cliente ja
//! mostra suas mensagens localmente).

use std::sync::{
    atomic::{AtomicU64, AtomicUsize, Ordering},
    Arc,
};

use bytes::Bytes;
use tokio::sync::broadcast;

use crate::protocol::ServerMsg;

/// Conteudo trafegado no barramento: JSON (texto/controle) ou bytes (imagem).
#[derive(Clone)]
pub enum Payload {
    Json(String),
    Binary(Bytes),
}

/// Envelope publicado: identifica a conexao de origem para evitar eco.
#[derive(Clone)]
pub struct Envelope {
    pub origin: u64,
    pub payload: Payload,
}

/// Origem reservada ao servidor (mensagens de sistema/presenca chegam a todos).
pub const SERVER_ORIGIN: u64 = 0;

/// Hub compartilhado entre todas as conexoes.
#[derive(Clone)]
pub struct Hub {
    tx: broadcast::Sender<Envelope>,
    next_id: Arc<AtomicU64>,
    count: Arc<AtomicUsize>,
}

impl Hub {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(1024);
        Self {
            tx,
            next_id: Arc::new(AtomicU64::new(1)),
            count: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Aloca um id unico para uma nova conexao.
    pub fn next_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }

    /// Assina o barramento para receber o que outros publicarem.
    pub fn subscribe(&self) -> broadcast::Receiver<Envelope> {
        self.tx.subscribe()
    }

    /// Publica um payload vindo de `origin` para todos os demais.
    pub fn publish(&self, origin: u64, payload: Payload) {
        let _ = self.tx.send(Envelope { origin, payload });
    }

    /// Publica uma mensagem do servidor (chega a todos, sem origem propria).
    pub fn publish_server(&self, msg: &ServerMsg) {
        self.publish(SERVER_ORIGIN, Payload::Json(msg.to_json()));
    }

    /// Registra entrada de cliente e dispara `Presence` atualizado.
    pub fn join(&self) {
        let n = self.count.fetch_add(1, Ordering::SeqCst) + 1;
        self.publish_server(&ServerMsg::Presence { count: n });
    }

    /// Registra saida de cliente e dispara `Presence` atualizado.
    pub fn leave(&self) {
        let prev = self.count.fetch_sub(1, Ordering::SeqCst);
        let n = prev.saturating_sub(1);
        self.publish_server(&ServerMsg::Presence { count: n });
    }
}

impl Default for Hub {
    fn default() -> Self {
        Self::new()
    }
}
