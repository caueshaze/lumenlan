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

const TOKEN_KEY = "lumenlan.token";

/// Resolve o token de sala desta sessao:
/// - celular: vem na URL (`?t=...`, do QR) e fica salvo para recarregar;
/// - janela nativa: vem pelo IPC do Tauri (`app_token`).
export async function resolveToken(): Promise<string> {
  const fromUrl = new URLSearchParams(location.search).get("t");
  if (fromUrl) {
    localStorage.setItem(TOKEN_KEY, fromUrl);
    return fromUrl;
  }
  if (isTauri()) {
    try {
      const invoke = (window as any).__TAURI__.core.invoke;
      return await invoke("app_token");
    } catch {
      return "";
    }
  }
  return localStorage.getItem(TOKEN_KEY) ?? "";
}

/// Salva o PIN digitado manualmente (PC abrindo a URL sem `?t=`).
export function storeToken(pin: string) {
  localStorage.setItem(TOKEN_KEY, pin);
}

export type ServerInfo = { ip: string; port: number };

/// IP e porta do servidor.
/// - Janela nativa (Tauri): via IPC, pois a webview nao faz `fetch` cross-origin.
/// - Navegador/celular: via `/health` (mesma origem do axum).
export async function getServerInfo(): Promise<ServerInfo> {
  if (isTauri()) {
    const invoke = (window as any).__TAURI__.core.invoke;
    return await invoke("server_info");
  }
  const res = await fetch(`${httpBase()}/health`, { cache: "no-store" });
  const h = await res.json();
  return { ip: h.ip, port: h.port };
}
