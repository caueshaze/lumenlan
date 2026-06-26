<script lang="ts">
  import { isPreviewableImage } from "../lib/files";
  import type { ChatItem } from "../lib/protocol";

  let { items }: { items: ChatItem[] } = $props();

  let scroller: HTMLDivElement;

  // Item alvo do menu (long-press no celular) e feedback de "copiado".
  let actionItem = $state<ChatItem | null>(null);
  let copied = $state(false);
  let pressTimer: ReturnType<typeof setTimeout> | undefined;
  let failedPreviews = $state<Record<string, true>>({});

  // Rola para o fim sempre que a lista muda.
  $effect(() => {
    items.length;
    if (scroller) scroller.scrollTop = scroller.scrollHeight;
  });

  function time(ts: number): string {
    return new Date(ts).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
  }

  function humanSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    const units = ["KB", "MB", "GB", "TB"];
    let v = bytes / 1024;
    let i = 0;
    while (v >= 1024 && i < units.length - 1) {
      v /= 1024;
      i++;
    }
    return `${v.toFixed(1)} ${units[i]}`;
  }

  // Texto copiavel: corpo da mensagem (texto) ou nome do arquivo.
  function copyValue(item: ChatItem): string {
    if (item.kind === "text") return item.body;
    if (item.kind === "file") return item.name;
    return "";
  }

  // Fallback para contexto inseguro (celular via http://, sem navigator.clipboard).
  function legacyCopy(text: string): boolean {
    const ta = document.createElement("textarea");
    ta.value = text;
    ta.style.position = "fixed";
    ta.style.opacity = "0";
    document.body.appendChild(ta);
    ta.focus();
    ta.select();
    let ok = false;
    try {
      ok = document.execCommand("copy");
    } catch {
      ok = false;
    }
    document.body.removeChild(ta);
    return ok;
  }

  async function copy(item: ChatItem) {
    const text = copyValue(item);
    let ok = false;
    try {
      if (navigator.clipboard?.writeText) {
        await navigator.clipboard.writeText(text);
        ok = true;
      }
    } catch {
      ok = false;
    }
    if (!ok) ok = legacyCopy(text);
    if (ok) {
      copied = true;
      setTimeout(() => (copied = false), 1200);
    }
    actionItem = null;
  }

  // Long-press (apenas toque) abre o menu de copiar.
  function pressStart(e: PointerEvent, item: ChatItem) {
    if (e.pointerType !== "touch") return;
    pressTimer = setTimeout(() => (actionItem = item), 450);
  }
  function pressEnd() {
    clearTimeout(pressTimer);
  }

  function canPreview(item: ChatItem): boolean {
    return item.kind === "file" && isPreviewableImage(item.mime, item.name) && !failedPreviews[item.url];
  }

  function markPreviewFailed(url: string) {
    failedPreviews = { ...failedPreviews, [url]: true };
  }
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && (actionItem = null)} />

<div class="list" bind:this={scroller}>
  {#each items as item (item.ts + (item.kind === "text" ? item.body : ""))}
    {#if item.kind === "system"}
      <div class="system">{item.body}</div>
    {:else}
      <div
        class="bubble"
        class:mine={item.mine}
        class:file={item.kind === "file"}
        role="group"
        onpointerdown={(e) => pressStart(e, item)}
        onpointerup={pressEnd}
        onpointermove={pressEnd}
        onpointercancel={pressEnd}
        onpointerleave={pressEnd}
      >
        {#if !item.mine}<span class="from">{item.from}</span>{/if}

        {#if item.kind === "text"}
          <span class="body">{item.body}</span>
        {:else if canPreview(item)}
          <a href={item.url} download={item.name} target="_blank" rel="noopener" title="Abrir / baixar">
            <img src={item.url} alt={item.name} onerror={() => markPreviewFailed(item.url)} />
          </a>
          <span class="name">{item.name}</span>
        {:else}
          <a class="filechip" href={item.url} download={item.name} title="Baixar">
            <span class="ficon">📄</span>
            <span class="finfo">
              <span class="fname">{item.name}</span>
              <span class="fsize">{humanSize(item.size)}</span>
            </span>
            <span class="fdl">⬇</span>
          </a>
        {/if}

        <span class="ts">{time(item.ts)}</span>

        <button class="copy" title="Copiar" aria-label="Copiar" onclick={() => copy(item)}>⧉</button>
      </div>
    {/if}
  {/each}
</div>

{#if actionItem}
  <div class="sheet-overlay" role="presentation" onclick={(e) => e.target === e.currentTarget && (actionItem = null)}>
    <div class="sheet" role="dialog" aria-modal="true" aria-label="Acoes da mensagem" tabindex="-1">
      <button onclick={() => actionItem && copy(actionItem)}>⧉ Copiar</button>
      <button class="cancel" onclick={() => (actionItem = null)}>Cancelar</button>
    </div>
  </div>
{/if}

{#if copied}
  <div class="toast">Copiado!</div>
{/if}

<style>
  .list {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  .system {
    align-self: center;
    font-size: 0.75rem;
    color: var(--text-dim);
    padding: 0.15rem 0.6rem;
  }
  .bubble {
    position: relative;
    align-self: flex-start;
    max-width: 75%;
    background: var(--bg-elev);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 0.45rem 0.7rem;
    display: flex;
    flex-direction: column;
    gap: 0.15rem;
  }
  .bubble.mine {
    align-self: flex-end;
    background: var(--accent-soft);
    border-color: transparent;
  }
  .from {
    font-size: 0.72rem;
    color: var(--accent);
    font-weight: 600;
  }
  .body {
    white-space: pre-wrap;
    word-break: break-word;
  }
  .file img {
    max-width: 320px;
    max-height: 320px;
    border-radius: 8px;
    display: block;
  }
  .name {
    font-size: 0.7rem;
    color: var(--text-dim);
    word-break: break-all;
  }
  .filechip {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    text-decoration: none;
    color: var(--text);
    min-width: 12rem;
    max-width: 18rem;
  }
  .ficon {
    font-size: 1.6rem;
  }
  .finfo {
    display: flex;
    flex-direction: column;
    min-width: 0;
    flex: 1;
  }
  .fname {
    word-break: break-all;
    font-size: 0.9rem;
  }
  .fsize {
    font-size: 0.72rem;
    color: var(--text-dim);
  }
  .fdl {
    color: var(--accent);
    font-size: 1.1rem;
  }
  .ts {
    align-self: flex-end;
    font-size: 0.65rem;
    color: var(--text-dim);
  }

  /* Botao copiar: aparece ao passar o mouse (desktop). No toque usamos o menu. */
  .copy {
    position: absolute;
    top: -10px;
    right: -8px;
    width: 26px;
    height: 26px;
    border-radius: 50%;
    border: 1px solid var(--border);
    background: var(--bg);
    color: var(--text);
    font-size: 0.8rem;
    line-height: 1;
    display: grid;
    place-items: center;
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.12s ease;
  }
  .bubble:hover .copy {
    opacity: 1;
    pointer-events: auto;
  }
  .copy:hover {
    border-color: var(--accent);
    color: var(--accent);
  }

  /* Menu de long-press (celular) */
  .sheet-overlay {
    position: fixed;
    inset: 0;
    background: #0008;
    display: flex;
    align-items: flex-end;
    justify-content: center;
    z-index: 20;
  }
  .sheet {
    width: min(96vw, 420px);
    margin: 0.6rem;
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }
  .sheet button {
    padding: 0.9rem;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--bg-elev);
    color: var(--text);
    font: inherit;
    font-weight: 600;
  }
  .sheet .cancel {
    color: var(--text-dim);
  }

  .toast {
    position: fixed;
    bottom: 4.5rem;
    left: 50%;
    transform: translateX(-50%);
    background: #1f2937;
    color: #fff;
    padding: 0.45rem 0.9rem;
    border-radius: 999px;
    font-size: 0.82rem;
    z-index: 30;
    box-shadow: 0 4px 16px #0006;
  }
</style>
