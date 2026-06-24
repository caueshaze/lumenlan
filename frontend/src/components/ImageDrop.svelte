<script lang="ts">
  let { onpick, disabled = false }: { onpick: (file: File) => void; disabled?: boolean } =
    $props();

  let input: HTMLInputElement;
  let dragging = $state(false);

  function pickFiles(files: FileList | null | undefined) {
    if (!files) return;
    for (const f of files) {
      if (f.type.startsWith("image/")) onpick(f);
    }
  }

  function onchange(e: Event) {
    pickFiles((e.target as HTMLInputElement).files);
    input.value = "";
  }

  function ondrop(e: DragEvent) {
    e.preventDefault();
    dragging = false;
    if (!disabled) pickFiles(e.dataTransfer?.files);
  }
</script>

<div
  class="drop"
  class:dragging
  role="button"
  tabindex="0"
  ondragover={(e) => {
    e.preventDefault();
    dragging = true;
  }}
  ondragleave={() => (dragging = false)}
  ondrop={ondrop}
  onclick={() => !disabled && input.click()}
  onkeydown={(e) => e.key === "Enter" && !disabled && input.click()}
>
  <span>📷 Arraste uma imagem ou clique para enviar</span>
  <input bind:this={input} type="file" accept="image/*" multiple {onchange} hidden />
</div>

<style>
  .drop {
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
</style>
