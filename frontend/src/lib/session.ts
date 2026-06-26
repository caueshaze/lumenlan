import type { ChatItem, ServerMsg } from "./protocol";
import type { IncomingFile } from "./files";

export type SessionViewState = {
  onboarded: boolean;
  name: string;
  items: ChatItem[];
  connected: boolean;
  presence: number;
  showConnect: boolean;
  token: string;
  gated: boolean;
  authError: string;
};

export function initialViewState(storedName: string | null): SessionViewState {
  return {
    onboarded: storedName !== null,
    name: storedName ?? randomDeviceName(),
    items: [],
    connected: false,
    presence: 0,
    showConnect: false,
    token: "",
    gated: false,
    authError: "",
  };
}

export function sanitizeDisplayName(raw: string): string {
  return raw.trim() || "anon";
}

export function randomDeviceName(): string {
  return `Dispositivo-${Math.floor(Math.random() * 0xffff).toString(16)}`;
}

export function incomingTextItem(msg: Extract<ServerMsg, { type: "text" }>): ChatItem {
  return { kind: "text", mine: false, from: msg.from, body: msg.body, ts: msg.ts };
}

export function incomingSystemItem(body: string, ts = Date.now()): ChatItem {
  return { kind: "system", body, ts };
}

export function incomingFileItem(file: IncomingFile, ts = Date.now()): ChatItem {
  return {
    kind: "file",
    mine: false,
    from: file.from,
    name: file.name,
    mime: file.mime,
    size: file.size,
    url: file.toUrl(),
    ts,
  };
}

export function outgoingTextItem(name: string, body: string, ts = Date.now()): ChatItem {
  return { kind: "text", mine: true, from: name, body, ts };
}

export function outgoingFileItem(
  name: string,
  file: File,
  url: string,
  ts = Date.now(),
): ChatItem {
  return {
    kind: "file",
    mine: true,
    from: name,
    name: file.name,
    mime: file.type || "application/octet-stream",
    size: file.size,
    url,
    ts,
  };
}

export function fileSendFailureItem(fileName: string, ts = Date.now()): ChatItem {
  return {
    kind: "system",
    body: `falha ao enviar "${fileName}"`,
    ts,
  };
}
