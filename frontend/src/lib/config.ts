// Resolve as URLs do servidor conforme o contexto de execucao.
// - Janela nativa (Tauri): assets vem de tauri://, entao apontamos para localhost.
// - Navegador/celular: a propria pagina ja foi servida pelo axum (mesma origem).

export const SERVER_PORT = 8787;

export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export function httpBase(): string {
  if (isTauri()) return `http://localhost:${SERVER_PORT}`;
  return `${location.protocol}//${location.host}`;
}

export function wsBase(): string {
  if (isTauri()) return `ws://localhost:${SERVER_PORT}`;
  const proto = location.protocol === "https:" ? "wss:" : "ws:";
  return `${proto}//${location.host}`;
}
