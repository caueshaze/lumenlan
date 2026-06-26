<script lang="ts">
  let { onsubmit, error = "" }: { onsubmit: (pin: string) => void; error?: string } = $props();

  let pin = $state("");

  function submit() {
    const clean = pin.replace(/\D/g, "").slice(0, 6);
    if (clean.length === 6) onsubmit(clean);
  }
</script>

<div class="gate">
  <div class="card">
    <h1>LumenLan</h1>
    <p>Digite o PIN exibido no computador anfitriao para entrar na sala.</p>
    <input
      class="pin"
      bind:value={pin}
      inputmode="numeric"
      maxlength="6"
      placeholder="000000"
      onkeydown={(e) => e.key === "Enter" && submit()}
      aria-label="PIN de sala"
    />
    {#if error}<p class="error">{error}</p>{/if}
    <button onclick={submit} disabled={pin.replace(/\D/g, "").length !== 6}>Entrar</button>
  </div>
</div>

<style>
  .gate {
    flex: 1;
    display: grid;
    place-items: center;
    padding: 1rem;
  }
  .card {
    text-align: center;
    max-width: 20rem;
    display: flex;
    flex-direction: column;
    gap: 0.8rem;
  }
  h1 {
    margin: 0;
    font-size: 1.8rem;
  }
  p {
    margin: 0;
    color: var(--text-dim);
    font-size: 0.9rem;
  }
  .pin {
    font-size: 2rem;
    letter-spacing: 0.5rem;
    text-align: center;
    font-family: ui-monospace, monospace;
    color: var(--text);
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 0.5rem;
  }
  .pin:focus {
    outline: none;
    border-color: var(--accent);
  }
  .error {
    color: #ff8a8a;
  }
  button {
    padding: 0.6rem;
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
