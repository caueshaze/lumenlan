<script lang="ts">
  let { onpick, disabled = false }: { onpick: (file: File) => void; disabled?: boolean } =
    $props();

  let dragging = $state(false);

  function pickFiles(files: FileList | null | undefined) {
    if (!files) return;
    for (let i = 0; i < files.length; i += 1) {
      const file = files.item(i);
      if (file) onpick(file);
    }
  }

  function onchange(e: Event) {
    const input = e.target as HTMLInputElement;
    pickFiles(input.files);
    input.value = "";
  }

  function ondrop(e: DragEvent) {
    e.preventDefault();
    dragging = false;
    if (!disabled) pickFiles(e.dataTransfer?.files);
  }
</script>

<label
  class="drop"
  class:dragging
  ondragover={(e) => {
    e.preventDefault();
    dragging = true;
  }}
  ondragleave={() => (dragging = false)}
  ondrop={ondrop}
>
  <span>📎 Arraste um arquivo ou toque para enviar</span>
  <span class="picker-label">Selecionar arquivos</span>
  <input
    class="picker-input"
    type="file"
    multiple
    accept="image/*,video/*,*/*"
    disabled={disabled}
    {onchange}
    aria-label="Selecionar arquivos"
  />
</label>

<style>
  .drop {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    margin: 0 0.9rem;
    padding: 0.5rem;
    border: 1px dashed var(--border);
    border-radius: var(--radius);
    text-align: center;
    font-size: 0.82rem;
    color: var(--text-dim);
    cursor: pointer;
    user-select: none;
  }
  .drop:hover,
  .drop.dragging {
    border-color: var(--accent);
    color: var(--text);
  }
  .picker-label {
    margin-top: 0.45rem;
    min-height: 2.25rem;
    padding: 0.55rem 0.85rem;
    border: 1px solid var(--border);
    border-radius: 999px;
    background: var(--bg);
    color: var(--text);
    font-size: 0.82rem;
  }
  .picker-input {
    position: absolute;
    inset: 0;
    opacity: 0;
    cursor: pointer;
  }
  .picker-input:disabled {
    cursor: not-allowed;
  }
  .drop:has(.picker-input:disabled) {
    opacity: 0.45;
  }
</style>
