<script lang="ts">
  import { untrack } from "svelte";

  let { defaultName, onfinish }: { defaultName: string; onfinish: (name: string) => void } =
    $props();

  // Valor inicial sugerido; o usuario edita livremente depois.
  let name = $state(untrack(() => defaultName));

  function start() {
    const clean = name.trim().slice(0, 32);
    if (clean) onfinish(clean);
  }
</script>

<div class="welcome">
  <div class="card">
    <img class="logo" src="/icons/icon-192.png" alt="LumenLan" />
    <h1>Bem-vindo ao LumenLan</h1>
    <p class="lead">
      Converse e troque imagens com aparelhos na mesma rede Wi-Fi — sem internet,
      sem cadastro, sem nuvem.
    </p>

    <ul class="features">
      <li>💬 Chat em tempo real com vários aparelhos</li>
      <li>🖼️ Envio de imagens direto pela rede local</li>
      <li>📱 Outros entram escaneando o QR ou digitando o PIN</li>
    </ul>

    <label class="field">
      <span>Nome do dispositivo</span>
      <input
        bind:value={name}
        maxlength="32"
        placeholder="ex.: Notebook da sala"
        onkeydown={(e) => e.key === "Enter" && start()}
        aria-label="Nome do dispositivo"
      />
    </label>

    <button onclick={start} disabled={!name.trim()}>Começar</button>
  </div>
</div>

<style>
  .welcome {
    flex: 1;
    display: grid;
    place-items: center;
    padding: 1.5rem;
    overflow-y: auto;
  }
  .card {
    width: min(92vw, 26rem);
    display: flex;
    flex-direction: column;
    gap: 0.85rem;
    text-align: center;
  }
  .logo {
    width: 72px;
    height: 72px;
    align-self: center;
    border-radius: 18px;
  }
  h1 {
    margin: 0;
    font-size: 1.5rem;
  }
  .lead {
    margin: 0;
    color: var(--text-dim);
    line-height: 1.5;
  }
  .features {
    list-style: none;
    margin: 0.2rem 0;
    padding: 0;
    text-align: left;
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    font-size: 0.92rem;
  }
  .field {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    text-align: left;
    margin-top: 0.3rem;
  }
  .field span {
    font-size: 0.8rem;
    color: var(--text-dim);
  }
  .field input {
    font: inherit;
    color: var(--text);
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 0.6rem 0.7rem;
  }
  .field input:focus {
    outline: none;
    border-color: var(--accent);
  }
  button {
    margin-top: 0.4rem;
    padding: 0.7rem;
    border: none;
    border-radius: var(--radius);
    background: var(--accent);
    color: #fff;
    font-weight: 600;
    font-size: 1rem;
  }
  button:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }
</style>
