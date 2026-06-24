<script lang="ts">
  import QRCode from "qrcode";
  import { httpBase, SERVER_PORT } from "../lib/config";

  let { onclose }: { onclose: () => void } = $props();

  let url = $state("");
  let qr = $state("");
  let error = $state("");

  async function load() {
    try {
      const res = await fetch(`${httpBase()}/health`, { cache: "no-store" });
      const h: { ip: string; port: number } = await res.json();
      url = `http://${h.ip}:${h.port}`;
      qr = await QRCode.toDataURL(url, { margin: 1, width: 240, color: { dark: "#0f1117", light: "#ffffff" } });
    } catch {
      error = "nao foi possivel obter o endereco do servidor";
    }
  }

  $effect(() => {
    load();
  });
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && onclose()} />

<div
  class="overlay"
  role="presentation"
  onclick={(e) => e.target === e.currentTarget && onclose()}
>
  <div class="modal" role="dialog" aria-modal="true" aria-label="Conectar celular" tabindex="-1">
    <h2>Conectar celular</h2>
    <p class="hint">Escaneie o QR ou abra o endereco no navegador do celular (mesma rede Wi-Fi).</p>

    {#if error}
      <p class="error">{error}</p>
    {:else if qr}
      <img class="qr" src={qr} alt="QR code do endereco" />
      <code class="url">{url}</code>
      <p class="hint alt">Em redes compativeis: <code>http://lumenlan.local:{SERVER_PORT}</code></p>
    {:else}
      <p class="hint">gerando QR...</p>
    {/if}

    <button class="close" onclick={onclose}>Fechar</button>
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: #000a;
    display: grid;
    place-items: center;
    z-index: 10;
  }
  .modal {
    background: var(--bg-elev);
    border: 1px solid var(--border);
    border-radius: 16px;
    padding: 1.5rem;
    width: min(92vw, 340px);
    text-align: center;
    display: flex;
    flex-direction: column;
    gap: 0.7rem;
  }
  h2 {
    margin: 0;
    font-size: 1.1rem;
  }
  .hint {
    margin: 0;
    font-size: 0.82rem;
    color: var(--text-dim);
  }
  .hint.alt {
    font-size: 0.75rem;
  }
  .qr {
    width: 240px;
    height: 240px;
    align-self: center;
    border-radius: 10px;
    background: #fff;
  }
  .url {
    color: var(--accent);
    background: var(--accent-soft);
    padding: 0.3rem 0.5rem;
    border-radius: 8px;
    word-break: break-all;
  }
  .error {
    color: #ff8a8a;
  }
  .close {
    margin-top: 0.3rem;
    padding: 0.5rem;
    border: none;
    border-radius: 10px;
    background: var(--accent);
    color: #fff;
    font-weight: 600;
  }
</style>
