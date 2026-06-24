// Envio e remontagem de imagens via WebSocket.
//
// Envio: FileStart (JSON) -> N chunks binarios crus -> FileEnd (JSON).
// Os chunks sao lidos do Blob por fatia (Blob.slice), evitando carregar o
// arquivo inteiro na memoria.
//
// Recepcao: o servidor reenvia cada chunk enquadrado como
// [id_len: u8][id utf8][dados]; aqui remontamos por `id`.

import type { LumenSocket } from "./ws";

const CHUNK = 64 * 1024;

/// Limite de tamanho por imagem (espelha MAX_FILE_BYTES do servidor): 50 MiB.
export const MAX_FILE_BYTES = 50 * 1024 * 1024;

/// Envia uma imagem em chunks. Retorna um object URL para preview local.
export async function sendImage(socket: LumenSocket, file: File): Promise<string> {
  const id = crypto.randomUUID();
  socket.send({
    type: "file_start",
    id,
    name: file.name,
    mime: file.type || "application/octet-stream",
    size: file.size,
  });

  for (let offset = 0; offset < file.size; offset += CHUNK) {
    const slice = file.slice(offset, offset + CHUNK);
    socket.sendBinary(await slice.arrayBuffer());
  }

  socket.send({ type: "file_end", id });
  return URL.createObjectURL(file);
}

/// Resultado do parse de um frame binario do servidor.
export type Frame = { id: string; data: Uint8Array };

export function parseFrame(buf: ArrayBuffer): Frame {
  const view = new Uint8Array(buf);
  const idLen = view[0];
  const id = new TextDecoder().decode(view.subarray(1, 1 + idLen));
  const data = view.subarray(1 + idLen);
  return { id, data };
}

/// Acumula chunks de uma imagem ate o FileEnd.
export class IncomingImage {
  readonly from: string;
  readonly name: string;
  readonly mime: string;
  private chunks: Uint8Array[] = [];

  constructor(from: string, name: string, mime: string) {
    this.from = from;
    this.name = name;
    this.mime = mime;
  }

  push(data: Uint8Array) {
    // Copia para desacoplar da view sobre o ArrayBuffer recebido.
    this.chunks.push(new Uint8Array(data));
  }

  /// Monta o Blob final e devolve um object URL pronto para <img>.
  toUrl(): string {
    const blob = new Blob(this.chunks as BlobPart[], { type: this.mime });
    return URL.createObjectURL(blob);
  }
}
