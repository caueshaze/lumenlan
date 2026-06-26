<script lang="ts">
  import { onMount } from "svelte";
  import StatusBar from "./components/StatusBar.svelte";
  import MessageList from "./components/MessageList.svelte";
  import MessageInput from "./components/MessageInput.svelte";
  import FileDrop from "./components/FileDrop.svelte";
  import ConnectModal from "./components/ConnectModal.svelte";
  import PinGate from "./components/PinGate.svelte";
  import Welcome from "./components/Welcome.svelte";
  import { createSessionController } from "./lib/session.svelte";

  const version = "0.1.0";
  const session = createSessionController();
  const state = session.state;

  onMount(() => {
    session.init();
    return () => session.destroy();
  });
</script>

{#if !state.onboarded}
  <Welcome defaultName={state.name} onfinish={session.finishWelcome} />
{:else if state.gated}
  <PinGate onsubmit={session.submitPin} error={state.authError} />
{:else}
  <header class="topbar">
    <span class="brand">LumenLan</span>
    <span class="tag">v{version}</span>
    <span class="spacer"></span>
    <button class="connect" onclick={session.openConnectModal} title="Conectar celular">
      📱 Conectar
    </button>
    <span class="presence" title="conectados">● {state.presence}</span>
    <input
      class="name"
      bind:value={state.name}
      onblur={session.rename}
      maxlength="32"
      aria-label="Seu nome"
    />
  </header>

  {#if !state.connected}
    <div class="banner">Reconectando ao servidor…</div>
  {/if}

  <main class="chat">
    <MessageList items={state.items} />
    <FileDrop onpick={session.pickFile} disabled={!state.connected} />
    <MessageInput onsend={session.sendText} disabled={!state.connected} />
  </main>

  <StatusBar />
{/if}

{#if state.showConnect}
  <ConnectModal token={state.token} onclose={session.closeConnectModal} />
{/if}

<style>
  .topbar {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.6rem 1rem;
    border-bottom: 1px solid var(--border);
    background: var(--bg-elev);
  }
  .brand {
    font-weight: 700;
    letter-spacing: 0.02em;
  }
  .tag {
    font-size: 0.72rem;
    color: var(--text-dim);
  }
  .spacer {
    flex: 1;
  }
  .connect {
    font-size: 0.78rem;
    color: var(--text);
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 0.25rem 0.6rem;
  }
  .connect:hover {
    border-color: var(--accent);
  }
  .presence {
    font-size: 0.8rem;
    color: #3ddc84;
  }
  .name {
    width: 9rem;
    font: inherit;
    color: var(--text);
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 0.25rem 0.5rem;
  }
  .name:focus {
    outline: none;
    border-color: var(--accent);
  }
  .chat {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }
  .banner {
    background: #7a2a2a;
    color: #ffe3e3;
    text-align: center;
    font-size: 0.8rem;
    padding: 0.3rem;
  }
</style>
