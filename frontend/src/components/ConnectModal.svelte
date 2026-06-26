<script lang="ts">
  import QRCode from "qrcode";
  import { getServerInfo, SERVER_PORT } from "../lib/config";

  let { token, onclose }: { token: string; onclose: () => void } = $props();

  let url = $state("");
  let qr = $state("");
  let error = $state("");

  async function load() {
    try {
      const info = await getServerInfo();
      // O token vai na URL do QR -> so quem escaneia entra na sala.
      url = `http://${info.ip}:${info.port}/?t=${encodeURIComponent(token)}`;
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

    {#if token}
      <div class="pin">PIN <strong>{token}</strong></div>
    {/if}

    {#if error}
      <p class="error">{error}</p>
    {:else if qr}
      <img class="qr" src={qr} alt="QR code do endereco" />
      <code class="url">{url}</code>
      <p class="hint alt">
        Em redes compativeis tambem vale
        <code>http://lumenlan.local:{SERVER_PORT}/?t={token}</code>
      </p>
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
  .pin {
    font-size: 0.85rem;
    color: var(--text-dim);
  }
  .pin strong {
    font-family: ui-monospace, monospace;
    font-size: 1.5rem;
    letter-spacing: 0.3rem;
    color: var(--text);
    margin-left: 0.3rem;
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
