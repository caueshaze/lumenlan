<script lang="ts">
  import { onMount } from "svelte";
  import { getServerInfo } from "../lib/config";

  let online = $state(false);
  let shareUrl = $state("");

  async function ping() {
    try {
      const info = await getServerInfo();
      online = true;
      shareUrl = `http://${info.ip}:${info.port}`;
    } catch {
      online = false;
    }
  }

  onMount(() => {
    ping();
    const id = setInterval(ping, 5000);
    return () => clearInterval(id);
  });
</script>

<footer class="statusbar">
  <span class="dot" class:online></span>
  <span class="label">{online ? "servidor no ar" : "servidor offline"}</span>
  {#if shareUrl}
    <span class="share">Celular: <code>{shareUrl}</code></span>
  {/if}
</footer>

<style>
  .statusbar {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.4rem 0.9rem;
    border-top: 1px solid var(--border);
    background: var(--bg-elev);
    font-size: 0.8rem;
    color: var(--text-dim);
  }
  .dot {
    width: 9px;
    height: 9px;
    border-radius: 50%;
    background: #6b7280;
    box-shadow: 0 0 0 0 transparent;
  }
  .dot.online {
    background: #3ddc84;
    box-shadow: 0 0 8px #3ddc8480;
  }
  .share {
    margin-left: auto;
  }
  code {
    color: var(--accent);
    background: var(--accent-soft);
    padding: 0.05rem 0.4rem;
    border-radius: 6px;
  }
</style>
