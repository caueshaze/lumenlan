// Espelha o contrato serde do servidor (src-tauri/src/protocol/message.rs).

export type ClientMsg =
  | { type: "hello"; name: string }
  | { type: "text"; body: string }
  | { type: "file_start"; id: string; name: string; mime: string; size: number }
  | { type: "file_end"; id: string };

export type ServerMsg =
  | { type: "text"; from: string; body: string; ts: number }
  | { type: "presence"; count: number }
  | { type: "system"; body: string }
  | { type: "file_start"; from: string; id: string; name: string; mime: string; size: number }
  | { type: "file_end"; id: string };

// Item renderizavel na lista de mensagens da UI.
export type ChatItem =
  | { kind: "text"; mine: boolean; from: string; body: string; ts: number }
  | { kind: "system"; body: string; ts: number }
  | { kind: "image"; mine: boolean; from: string; name: string; url: string; ts: number };
