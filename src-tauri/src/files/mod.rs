//! Recepcao de arquivos e enquadramento dos chunks no broadcast.

mod receiver;

use bytes::{BufMut, Bytes, BytesMut};

pub use receiver::Receiver;

/// Enquadra um chunk para o broadcast: `[id_len: u8][id utf8][dados]`.
///
/// O barramento e compartilhado entre todas as conexoes, entao chunks de
/// transferencias simultaneas podem se intercalar. O prefixo com o `id`
/// permite que cada cliente remonte a imagem correta. (Os clientes enviam
/// chunks crus ao servidor; a ordem por-conexao ja identifica a origem la.)
pub fn frame_chunk(id: &str, data: &[u8]) -> Bytes {
    let id_bytes = id.as_bytes();
    let id_len = id_bytes.len().min(255) as u8;
    let mut buf = BytesMut::with_capacity(1 + id_len as usize + data.len());
    buf.put_u8(id_len);
    buf.put_slice(&id_bytes[..id_len as usize]);
    buf.put_slice(data);
    buf.freeze()
}
