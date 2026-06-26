//! PIN de sala + protecao contra forca-bruta.
//!
//! A cada execucao o host gera um PIN curto (6 digitos), facil de digitar em
//! outro PC. Para um PIN curto ser seguro, limitamos tentativas erradas por IP:
//! apos algumas falhas, aquele IP fica em cooldown.

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Tentativas antes de travar um IP e duracao do bloqueio.
const MAX_FAILS: u32 = 8;
const LOCKOUT: Duration = Duration::from_secs(30);

/// Gera um PIN de 6 digitos a partir do RNG do sistema.
pub fn generate_pin() -> Arc<str> {
    let mut bytes = [0u8; 4];
    getrandom::getrandom(&mut bytes).expect("RNG do sistema indisponivel");
    let n = u32::from_le_bytes(bytes) % 1_000_000;
    Arc::from(format!("{n:06}").as_str())
}

#[derive(Default)]
struct Entry {
    fails: u32,
    locked_until: Option<Instant>,
}

/// Limitador de tentativas por IP (compartilhado entre conexoes).
#[derive(Clone, Default)]
pub struct Throttle {
    inner: Arc<Mutex<HashMap<IpAddr, Entry>>>,
}

impl Throttle {
    /// `true` se o IP pode tentar agora (nao esta em cooldown).
    pub fn allowed(&self, ip: IpAddr) -> bool {
        let mut map = self.inner.lock().unwrap();
        match map.get_mut(&ip) {
            Some(e) => match e.locked_until {
                Some(until) if Instant::now() < until => false,
                Some(_) => {
                    *e = Entry::default(); // cooldown expirou
                    true
                }
                None => true,
            },
            None => true,
        }
    }

    /// Registra uma tentativa malsucedida; ativa o cooldown ao atingir o limite.
    pub fn record_fail(&self, ip: IpAddr) {
        let mut map = self.inner.lock().unwrap();
        let e = map.entry(ip).or_default();
        e.fails += 1;
        if e.fails >= MAX_FAILS {
            e.locked_until = Some(Instant::now() + LOCKOUT);
            tracing::warn!("PIN: IP {ip} bloqueado por {}s apos {} falhas", LOCKOUT.as_secs(), e.fails);
        }
    }

    /// Limpa o estado de um IP que acertou o PIN.
    pub fn record_ok(&self, ip: IpAddr) {
        self.inner.lock().unwrap().remove(&ip);
    }
}
