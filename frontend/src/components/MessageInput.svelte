<script lang="ts">
  let { onsend, disabled = false }: { onsend: (body: string) => void; disabled?: boolean } =
    $props();

  let text = $state("");

  function submit() {
    const body = text.trim();
    if (!body) return;
    onsend(body);
    text = "";
  }

  function onkeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      submit();
    }
  }
</script>

<div class="input">
  <textarea
    bind:value={text}
    {onkeydown}
    placeholder="Mensagem... (Enter envia, Shift+Enter quebra linha)"
    rows="1"
  ></textarea>
  <button onclick={submit} disabled={disabled || !text.trim()}>Enviar</button>
</div>

<style>
  .input {
    display: flex;
    gap: 0.5rem;
    padding: 0.6rem 0.9rem;
    border-top: 1px solid var(--border);
    background: var(--bg-elev);
  }
  textarea {
    flex: 1;
    resize: none;
    font: inherit;
    color: var(--text);
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 0.5rem 0.7rem;
    max-height: 120px;
  }
  textarea:focus {
    outline: none;
    border-color: var(--accent);
  }
  button {
    padding: 0 1.1rem;
    border: none;
    border-radius: var(--radius);
    background: var(--accent);
    color: white;
    font-weight: 600;
  }
  button:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }
</style>
