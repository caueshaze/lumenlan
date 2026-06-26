import { IncomingFile, localFileUrl, parseFrame, sendFile } from "./files";
import { resolveToken, storeToken } from "./config";
import type { ChatItem, ServerMsg } from "./protocol";
import { LumenSocket } from "./ws";
import {
  fileSendFailureItem,
  incomingFileItem,
  incomingSystemItem,
  incomingTextItem,
  initialViewState,
  outgoingFileItem,
  outgoingTextItem,
  sanitizeDisplayName,
} from "./session";

const NAME_KEY = "lumenlan.name";
const AUTH_TIMEOUT_MS = 3500;

export function createSessionController() {
  const pending = new Map<string, IncomingFile>();
  const state = $state(initialViewState(readStoredName()));

  let socket: LumenSocket | undefined;
  let authTimer: ReturnType<typeof setTimeout> | undefined;
  let active = true;

  function add(item: ChatItem) {
    state.items = [...state.items, item];
  }

  function clearAuthTimer() {
    if (authTimer) {
      clearTimeout(authTimer);
      authTimer = undefined;
    }
  }

  function closeSocket() {
    socket?.close();
    socket = undefined;
  }

  function connectSocket() {
    clearAuthTimer();
    closeSocket();

    state.connected = false;
    socket = new LumenSocket(state.name, state.token);
    socket.onStatus = (open) => {
      state.connected = open;
      if (open) {
        clearAuthTimer();
      }
    };
    socket.onJson = (msg) => handleServerMessage(msg);
    socket.onBinary = (buf) => {
      const { id, data } = parseFrame(buf);
      pending.get(id)?.push(data);
    };
    socket.connect();

    authTimer = setTimeout(() => {
      if (!state.connected) {
        closeSocket();
        state.gated = true;
        state.authError = "PIN incorreto ou anfitriao indisponivel";
      }
    }, AUTH_TIMEOUT_MS);
  }

  function handleServerMessage(msg: ServerMsg) {
    switch (msg.type) {
      case "text":
        add(incomingTextItem(msg));
        break;
      case "presence":
        state.presence = msg.count;
        break;
      case "system":
        add(incomingSystemItem(msg.body));
        break;
      case "file_start":
        pending.set(msg.id, new IncomingFile(msg.from, msg.name, msg.mime, msg.size));
        break;
      case "file_end": {
        const file = pending.get(msg.id);
        pending.delete(msg.id);
        if (file) {
          add(incomingFileItem(file));
        }
        break;
      }
    }
  }

  function proceed() {
    if (state.token) {
      connectSocket();
    } else {
      state.gated = true;
    }
  }

  return {
    state,

    async init() {
      state.token = await resolveToken();
      if (!active) {
        return;
      }
      if (state.onboarded) {
        proceed();
      }
    },

    destroy() {
      active = false;
      clearAuthTimer();
      closeSocket();
      pending.clear();
    },

    finishWelcome(chosen: string) {
      state.name = chosen;
      localStorage.setItem(NAME_KEY, chosen);
      state.onboarded = true;
      proceed();
    },

    submitPin(pin: string) {
      state.token = pin;
      storeToken(pin);
      state.authError = "";
      state.gated = false;
      connectSocket();
    },

    async pickFile(file: File) {
      if (!socket?.isOpen) {
        add(incomingSystemItem("conexao indisponivel para enviar arquivo"));
        return;
      }

      const url = localFileUrl(file);
      add(outgoingFileItem(state.name, file, url));

      try {
        await sendFile(socket, file);
      } catch (error) {
        console.error("falha ao enviar arquivo", error);
        add(fileSendFailureItem(file.name));
      }
    },

    sendText(body: string) {
      socket?.send({ type: "text", body });
      add(outgoingTextItem(state.name, body));
    },

    rename() {
      const clean = sanitizeDisplayName(state.name);
      state.name = clean;
      localStorage.setItem(NAME_KEY, clean);
      socket?.send({ type: "hello", name: clean });
    },

    openConnectModal() {
      state.showConnect = true;
    },

    closeConnectModal() {
      state.showConnect = false;
    },
  };
}

function readStoredName(): string | null {
  return localStorage.getItem(NAME_KEY);
}
