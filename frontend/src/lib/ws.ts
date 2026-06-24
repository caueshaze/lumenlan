// Cliente WebSocket com reconexao automatica (backoff simples).
// Frames de texto carregam JSON (ServerMsg); frames binarios carregam chunks
// de imagem (tratados na Fase 3).

import { wsBase } from "./config";
import type { ClientMsg, ServerMsg } from "./protocol";

type JsonHandler = (msg: ServerMsg) => void;
type BinaryHandler = (data: ArrayBuffer) => void;
type StatusHandler = (open: boolean) => void;

export class LumenSocket {
  private ws: WebSocket | null = null;
  private closedByUser = false;
  private retry = 0;
  private name: string;

  onJson: JsonHandler = () => {};
  onBinary: BinaryHandler = () => {};
  onStatus: StatusHandler = () => {};

  constructor(name: string) {
    this.name = name;
  }

  connect() {
    this.closedByUser = false;
    const ws = new WebSocket(`${wsBase()}/ws`);
    ws.binaryType = "arraybuffer";
    this.ws = ws;

    ws.onopen = () => {
      this.retry = 0;
      this.onStatus(true);
      this.send({ type: "hello", name: this.name });
    };

    ws.onmessage = (ev) => {
      if (typeof ev.data === "string") {
        try {
          this.onJson(JSON.parse(ev.data) as ServerMsg);
        } catch {
          /* frame ignorado */
        }
      } else if (ev.data instanceof ArrayBuffer) {
        this.onBinary(ev.data);
      }
    };

    ws.onclose = () => {
      this.onStatus(false);
      if (!this.closedByUser) this.scheduleReconnect();
    };

    ws.onerror = () => ws.close();
  }

  private scheduleReconnect() {
    this.retry = Math.min(this.retry + 1, 6);
    const delay = Math.min(500 * 2 ** (this.retry - 1), 8000);
    setTimeout(() => {
      if (!this.closedByUser) this.connect();
    }, delay);
  }

  send(msg: ClientMsg) {
    this.ws?.readyState === WebSocket.OPEN && this.ws.send(JSON.stringify(msg));
  }

  sendBinary(data: ArrayBuffer | ArrayBufferView) {
    if (this.ws?.readyState === WebSocket.OPEN) this.ws.send(data as BufferSource);
  }

  get isOpen(): boolean {
    return this.ws?.readyState === WebSocket.OPEN;
  }

  close() {
    this.closedByUser = true;
    this.ws?.close();
  }
}
