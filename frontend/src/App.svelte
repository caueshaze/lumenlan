<script lang="ts">
  import { onMount } from "svelte";
  import StatusBar from "./components/StatusBar.svelte";
  import MessageList from "./components/MessageList.svelte";
  import MessageInput from "./components/MessageInput.svelte";
  import ImageDrop from "./components/ImageDrop.svelte";
  import ConnectModal from "./components/ConnectModal.svelte";
  import { LumenSocket } from "./lib/ws";
  import { sendImage, parseFrame, IncomingImage, MAX_FILE_BYTES } from "./lib/files";
  import type { ChatItem } from "./lib/protocol";

  const version = "0.1.0";

  // Nome persistente (editavel no topo). Default amigavel e aleatorio.
  let name = $state(
    localStorage.getItem("lumenlan.name") ??
      `Visitante-${Math.floor(Math.random() * 0xffff).toString(16)}`,
  );

  let items = $state<ChatItem[]>([]);
  let connected = $state(false);
  let presence = $state(0);
  let showConnect = $state(false);

  let socket: LumenSocket;

  // Imagens em recepcao, indexadas pelo id da transferencia.
  const pending = new Map<string, IncomingImage>();

  function add(item: ChatItem) {
    items = [...items, item];
  }

  onMount(() => {
    socket = new LumenSocket(name);
    socket.onStatus = (open) => (connected = open);
    socket.onJson = (msg) => {
      switch (msg.type) {
        case "text":
          add({ kind: "text", mine: false, from: msg.from, body: msg.body, ts: msg.ts });
          break;
        case "presence":
          presence = msg.count;
          break;
        case "system":
          add({ kind: "system", body: msg.body, ts: Date.now() });
          break;
        case "file_start":
          pending.set(msg.id, new IncomingImage(msg.from, msg.name, msg.mime));
          break;
        case "file_end": {
          const img = pending.get(msg.id);
          pending.delete(msg.id);
          if (img) {
            add({
              kind: "image",
              mine: false,
              from: img.from,
              name: img.name,
              url: img.toUrl(),
              ts: Date.now(),
            });
          }
          break;
        }
      }
    };
    socket.onBinary = (buf) => {
      const { id, data } = parseFrame(buf);
      pending.get(id)?.push(data);
    };
    socket.connect();
    return () => socket.close();
  });

  function sendText(body: string) {
    socket.send({ type: "text", body });
    add({ kind: "text", mine: true, from: name, body, ts: Date.now() });
  }

  async function onPickImage(file: File) {
    if (file.size > MAX_FILE_BYTES) {
      add({
        kind: "system",
        body: `"${file.name}" excede ${MAX_FILE_BYTES / 1024 / 1024} MB e nao foi enviada`,
        ts: Date.now(),
      });
      return;
    }
    const url = await sendImage(socket, file);
    add({ kind: "image", mine: true, from: name, name: file.name, url, ts: Date.now() });
  }

  function renameOnBlur() {
    const clean = name.trim() || "anon";
    name = clean;
    localStorage.setItem("lumenlan.name", clean);
    socket?.send({ type: "hello", name: clean });
  }
</script>

<header class="topbar">
  <span class="brand">LumenLan</span>
  <span class="tag">v{version}</span>
  <span class="spacer"></span>
  <button class="connect" onclick={() => (showConnect = true)} title="Conectar celular">
    📱 Conectar
  </button>
  <span class="presence" title="conectados">● {presence}</span>
  <input
    class="name"
    bind:value={name}
    onblur={renameOnBlur}
    maxlength="32"
    aria-label="Seu nome"
  />
</header>

{#if !connected}
  <div class="banner">Reconectando ao servidor…</div>
{/if}

<main class="chat">
  <MessageList {items} />
  <ImageDrop onpick={onPickImage} disabled={!connected} />
  <MessageInput onsend={sendText} disabled={!connected} />
</main>

<StatusBar />

{#if showConnect}
  <ConnectModal onclose={() => (showConnect = false)} />
{/if}

<style>
  .topbar {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.6rem 1rem;
    border-bottom: 1px solid var(--border);
    background: var(--bg-elev);
  }
  .brand {
    font-weight: 700;
    letter-spacing: 0.02em;
  }
  .tag {
    font-size: 0.72rem;
    color: var(--text-dim);
  }
  .spacer {
    flex: 1;
  }
  .connect {
    font-size: 0.78rem;
    color: var(--text);
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 0.25rem 0.6rem;
  }
  .connect:hover {
    border-color: var(--accent);
  }
  .presence {
    font-size: 0.8rem;
    color: #3ddc84;
  }
  .name {
    width: 9rem;
    font: inherit;
    color: var(--text);
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 0.25rem 0.5rem;
  }
  .name:focus {
    outline: none;
    border-color: var(--accent);
  }
  .chat {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }
  .banner {
    background: #7a2a2a;
    color: #ffe3e3;
    text-align: center;
    font-size: 0.8rem;
    padding: 0.3rem;
  }
</style>
