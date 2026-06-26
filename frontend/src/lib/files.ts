// Envio e remontagem de arquivos (qualquer tipo) via WebSocket.
//
// Envio: FileStart (JSON) -> N chunks binarios crus -> FileEnd (JSON).
// Os chunks sao lidos do Blob por fatia (Blob.slice), evitando carregar o
// arquivo inteiro na memoria.
//
// Recepcao: o servidor reenvia cada chunk enquadrado como
// [id_len: u8][id utf8][dados]; aqui remontamos por `id`.

import type { LumenSocket } from "./ws";

const CHUNK = 64 * 1024;
const PREVIEWABLE_IMAGE_MIMES = new Set([
  "image/avif",
  "image/bmp",
  "image/gif",
  "image/jpeg",
  "image/png",
  "image/svg+xml",
  "image/webp",
]);
const PREVIEWABLE_IMAGE_EXTENSIONS = [".avif", ".bmp", ".gif", ".jpg", ".jpeg", ".png", ".svg", ".webp"];

/// Envia um arquivo em chunks.
export async function sendFile(socket: LumenSocket, file: File): Promise<void> {
  const id = createTransferId();
  socket.send({
    type: "file_start",
    id,
    name: file.name,
    mime: file.type || "application/octet-stream",
    size: file.size,
  });

  for (let offset = 0; offset < file.size; offset += CHUNK) {
    const slice = file.slice(offset, offset + CHUNK);
    socket.sendBinary(await blobToArrayBuffer(slice));
  }

  socket.send({ type: "file_end", id });
}

export function localFileUrl(file: File): string {
  return URL.createObjectURL(file);
}

export function isPreviewableImage(mime: string, name: string): boolean {
  const normalizedMime = mime.trim().toLowerCase();
  if (PREVIEWABLE_IMAGE_MIMES.has(normalizedMime)) {
    return true;
  }

  const normalizedName = name.trim().toLowerCase();
  return PREVIEWABLE_IMAGE_EXTENSIONS.some((ext) => normalizedName.endsWith(ext));
}

async function blobToArrayBuffer(blob: Blob): Promise<ArrayBuffer> {
  if (typeof blob.arrayBuffer === "function") {
    return blob.arrayBuffer();
  }

  return await new Promise<ArrayBuffer>((resolve, reject) => {
    const reader = new FileReader();
    reader.onerror = () => reject(reader.error ?? new Error("falha ao ler arquivo"));
    reader.onload = () => resolve(reader.result as ArrayBuffer);
    reader.readAsArrayBuffer(blob);
  });
}

function createTransferId(): string {
  if (typeof crypto !== "undefined" && typeof crypto.randomUUID === "function") {
    return crypto.randomUUID();
  }

  return `file-${Date.now()}-${Math.random().toString(16).slice(2)}`;
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

/// Acumula chunks de um arquivo ate o FileEnd.
export class IncomingFile {
  readonly from: string;
  readonly name: string;
  readonly mime: string;
  readonly size: number;
  private chunks: Uint8Array[] = [];

  constructor(from: string, name: string, mime: string, size: number) {
    this.from = from;
    this.name = name;
    this.mime = mime;
    this.size = size;
  }

  push(data: Uint8Array) {
    // Copia para desacoplar da view sobre o ArrayBuffer recebido.
    this.chunks.push(new Uint8Array(data));
  }

  /// Monta o Blob final e devolve um object URL (download ou <img>).
  toUrl(): string {
    const blob = new Blob(this.chunks as BlobPart[], { type: this.mime });
    return URL.createObjectURL(blob);
  }
}
