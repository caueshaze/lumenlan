//! Gravacao em streaming de arquivos recebidos para `./lumenlan_received/`.
//!
//! Os chunks sao escritos no disco assim que chegam — nunca mantemos o arquivo
//! inteiro em RAM (requisito de baixo consumo de memoria).

use std::path::{Path, PathBuf};

use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

use crate::config;

/// Escritor de um arquivo em andamento.
pub struct Receiver {
    file: File,
    path: PathBuf,
    written: u64,
}

impl Receiver {
    /// Cria o arquivo de destino (criando a pasta se preciso), evitando
    /// sobrescrever: se o nome existir, adiciona um sufixo numerico.
    pub async fn create(name: &str) -> std::io::Result<Self> {
        let dir = config::download_dir();
        fs::create_dir_all(&dir).await?;
        let path = unique_path(&dir, &sanitize(name));
        let file = File::create(&path).await?;
        tracing::info!("recebendo arquivo em {}", path.display());
        Ok(Self {
            file,
            path,
            written: 0,
        })
    }

    /// Anexa um chunk ao arquivo.
    pub async fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.file.write_all(data).await?;
        self.written += data.len() as u64;
        Ok(())
    }

    /// Garante a persistencia no disco e devolve o caminho final.
    pub async fn finish(mut self) -> std::io::Result<PathBuf> {
        self.file.flush().await?;
        self.file.sync_all().await?;
        tracing::info!("arquivo salvo: {} ({} bytes)", self.path.display(), self.written);
        Ok(self.path)
    }
}

/// Remove separadores de caminho e caracteres de controle do nome do arquivo.
fn sanitize(name: &str) -> String {
    let base = name.rsplit(['/', '\\']).next().unwrap_or(name);
    let cleaned: String = base
        .chars()
        .filter(|c| !c.is_control() && *c != '/' && *c != '\\')
        .take(120)
        .collect();
    if cleaned.trim().is_empty() {
        "imagem".to_string()
    } else {
        cleaned
    }
}

/// Gera um caminho que ainda nao existe (`foo.png`, `foo-1.png`, ...).
fn unique_path(dir: &Path, name: &str) -> PathBuf {
    let candidate = dir.join(name);
    if !candidate.exists() {
        return candidate;
    }
    let (stem, ext) = match name.rsplit_once('.') {
        Some((s, e)) => (s.to_string(), format!(".{e}")),
        None => (name.to_string(), String::new()),
    };
    for n in 1.. {
        let candidate = dir.join(format!("{stem}-{n}{ext}"));
        if !candidate.exists() {
            return candidate;
        }
    }
    unreachable!()
}
