<script lang="ts">
  import type { ChatItem } from "../lib/protocol";

  let { items }: { items: ChatItem[] } = $props();

  let scroller: HTMLDivElement;

  // Rola para o fim sempre que a lista muda.
  $effect(() => {
    items.length;
    if (scroller) scroller.scrollTop = scroller.scrollHeight;
  });

  function time(ts: number): string {
    return new Date(ts).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
  }
</script>

<div class="list" bind:this={scroller}>
  {#each items as item (item.ts + (item.kind === "text" ? item.body : ""))}
    {#if item.kind === "system"}
      <div class="system">{item.body}</div>
    {:else if item.kind === "text"}
      <div class="bubble" class:mine={item.mine}>
        {#if !item.mine}<span class="from">{item.from}</span>{/if}
        <span class="body">{item.body}</span>
        <span class="ts">{time(item.ts)}</span>
      </div>
    {:else}
      <div class="bubble image" class:mine={item.mine}>
        {#if !item.mine}<span class="from">{item.from}</span>{/if}
        <a href={item.url} download={item.name} target="_blank" rel="noopener" title="Abrir / baixar">
          <img src={item.url} alt={item.name} />
        </a>
        <span class="name">{item.name}</span>
        <span class="ts">{time(item.ts)}</span>
      </div>
    {/if}
  {/each}
</div>

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
  .image img {
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
  .ts {
    align-self: flex-end;
    font-size: 0.65rem;
    color: var(--text-dim);
  }
</style>
