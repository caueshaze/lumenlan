<div align="center">

<img src="assets/logo.png" alt="LumenLan" width="128" height="128" />

# LumenLan

**Chat e compartilhamento de arquivos na sua rede local — sem internet, sem cadastro.**

O desktop (Linux/Windows) abre um app nativo e, ao mesmo tempo, serve a interface
para qualquer celular na mesma Wi-Fi. É só escanear o QR code.

[![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Tauri v2](https://img.shields.io/badge/Tauri-2-24C8DB?style=flat&logo=tauri&logoColor=white)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/Svelte-5-FF3E00?style=flat&logo=svelte&logoColor=white)](https://svelte.dev/)
[![Vite](https://img.shields.io/badge/Vite-6-646CFF?style=flat&logo=vite&logoColor=white)](https://vite.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?style=flat&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Tokio](https://img.shields.io/badge/Tokio-async-463AA1?style=flat&logo=rust&logoColor=white)](https://tokio.rs/)
[![axum](https://img.shields.io/badge/axum-WebSocket-000000?style=flat&logo=rust&logoColor=white)](https://github.com/tokio-rs/axum)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

</div>

---

## ✨ Recursos

- 💬 **Chat em tempo real** entre desktop e vários celulares (1 → N, broadcast).
- 📎 **Compartilhamento de arquivos** de qualquer tipo, **sem limite de tamanho**,
  em chunks e salvos automaticamente em `./lumenlan_received/` (imagens ganham preview).
- 📱 **PWA**: a interface web é instalável no celular (ícone na tela inicial).
- 🔌 **Binário único self-contained**: o frontend vai embutido no executável.
- 🛰️ **Descoberta automática** via mDNS (`lumenlan.local`) + **QR code** na janela nativa.
- ⚡ **Assíncrono** (Tokio): arquivos pesados não travam o chat de texto.

## 🏗️ Arquitetura

```
        Desktop (Linux/Windows)                         Celulares / outros PCs
   ┌───────────────────────────────┐
   │  App nativo (Tauri v2)         │                   ┌─────────────────────┐
   │  ┌─────────────────────────┐   │      Wi-Fi LAN     │  Navegador (PWA)    │
   │  │  UI Svelte (webview)     │   │   ◄───────────────►│  mesma UI Svelte    │
   │  └─────────────────────────┘   │   WebSocket /ws    └─────────────────────┘
   │  ┌─────────────────────────┐   │   HTTP  (axum)              ▲
   │  │  Servidor axum          │◄──┼── 0.0.0.0:8787 ─────────────┘
   │  │  /ws · /health · assets │   │
   │  └─────────────────────────┘   │
   └───────────────────────────────┘
```

A janela nativa e os celulares carregam **a mesma UI** e falam o mesmo protocolo
WebSocket. O servidor `axum` (embutido) faz o broadcast 1 → N e grava os arquivos
recebidos em disco via streaming.

## 🧰 Stack

| Camada | Tecnologia |
|--------|-----------|
| App nativo | **Tauri v2** (Linux + Windows) |
| Runtime async | **Tokio** |
| HTTP + WebSocket | **axum** (`tokio-tungstenite` por baixo) |
| Serialização | **serde** + **serde_json** |
| Assets embutidos | **rust-embed** |
| Descoberta | **mdns-sd** |
| Frontend | **Svelte 5** + **Vite** + **TypeScript** |

## 🚀 Rodando em desenvolvimento

**Pré-requisitos comuns:** [Rust](https://rustup.rs/), [Node 18+](https://nodejs.org/)
e `cargo install tauri-cli --version "^2"`.

### Dependências de sistema (Tauri v2)

<details>
<summary><strong>Ubuntu / Debian</strong></summary>

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```
</details>

<details>
<summary><strong>Arch / Manjaro</strong></summary>

```bash
sudo pacman -S --needed webkit2gtk-4.1 base-devel curl wget file openssl \
  libappindicator-gtk3 librsvg
```
</details>

<details>
<summary><strong>Fedora</strong></summary>

```bash
sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget file \
  libappindicator-gtk3-devel librsvg2-devel
sudo dnf group install "c-development"
```
</details>

<details>
<summary><strong>Windows</strong></summary>

- **Microsoft C++ Build Tools** (workload "Desktop development with C++").
- **WebView2** — já vem no Windows 11; no Windows 10 instale o
  [Evergreen Runtime](https://developer.microsoft.com/microsoft-edge/webview2/).

Nenhum pacote extra de GTK é necessário no Windows.
</details>

> Outras plataformas: veja os [pré-requisitos do Tauri](https://tauri.app/start/prerequisites/).

### Subindo o app

```bash
# 1. dependências do frontend
npm --prefix frontend install

# 2. roda o app nativo (o frontend sobe junto via beforeDevCommand)
cd src-tauri
cargo tauri dev
```

## 📱 Conectando o celular

Com o app aberto, clique em **📱 Conectar** para ver o **QR code** e o **PIN**.

- **Celular**: escaneie o QR — o PIN já vai embutido, entra direto.
- **Outro PC**: abra o endereço no navegador e **digite o PIN** na tela inicial:

```
http://<ip-do-desktop>:8787      # ex.: http://192.168.15.5:8787
http://lumenlan.local:8787       # em redes com suporte a mDNS (sem decorar o IP)
```

Qualquer dispositivo na mesma Wi-Fi conecta (vários ao mesmo tempo) — só pelo
navegador, sem instalar nada.

> **PWA:** no iOS use *Adicionar à Tela de Início* (Safari). No Android, a
> instalação completa de PWA exige HTTPS; em HTTP na LAN funciona como atalho.

### 🧱 Firewall

O host precisa **liberar a porta `8787/tcp`** para receber conexões — senão a
janela nativa abre (localhost), mas celular/outro PC ficam em *"servidor não
encontrado"*. Opcionalmente, `5353/udp` (mDNS) para o `lumenlan.local` funcionar.

<details>
<summary><strong>Fedora / openSUSE (firewalld)</strong></summary>

```bash
sudo firewall-cmd --add-port=8787/tcp --permanent
sudo firewall-cmd --add-port=5353/udp --permanent   # opcional (mDNS)
sudo firewall-cmd --reload
```
</details>

<details>
<summary><strong>Ubuntu / Debian (ufw)</strong></summary>

```bash
sudo ufw allow 8787/tcp
sudo ufw allow 5353/udp   # opcional (mDNS)
```
</details>

<details>
<summary><strong>Arch / Manjaro</strong></summary>

Sem firewall por padrão — costuma funcionar direto. Se você usa `ufw` ou
`firewalld`, aplique a regra equivalente acima.
</details>

<details>
<summary><strong>Windows</strong></summary>

Na primeira execução o **Windows Defender Firewall** mostra um aviso —
marque **Redes privadas** e clique em *Permitir acesso*. Se tiver clicado em
bloquear antes, libere em *Firewall do Windows → Permitir um aplicativo*.
</details>

## 🔒 Segurança

- **PIN de sala**: a cada execução o host gera um **PIN de 6 dígitos**. Só entra
  no chat quem tem o PIN — ele aparece no QR code (celular escaneia) e na tela do
  host (para digitar em outro PC), nunca em um endpoint público. Conexões ao
  `/ws` sem o PIN correto recebem `401`.
- **Anti-brute-force**: como o PIN é curto, tentativas erradas são limitadas por
  IP — após algumas falhas o IP entra em *cooldown* (`429`), inviabilizando
  adivinhação por força bruta.
- **Sanitização**: nomes de arquivo recebidos são limpos (sem path traversal) —
  não escapam de `./lumenlan_received/`.

> **Sem limite de tamanho**: arquivos de qualquer tipo/tamanho são aceitos; em
> redes de confiança isso é prático, mas lembre que um arquivo muito grande (ou
> muitos) pode encher o disco do host.

> **Escopo**: o tráfego na LAN é em texto puro (sem TLS), então o PIN protege
> contra alguém apenas *apontar o navegador* para o seu IP, mas não contra
> captura ativa de pacotes na mesma rede. Para uso doméstico é adequado; em redes
> não confiáveis, considere HTTPS/WSS (não incluso).

## 📦 Build de produção

```bash
cd src-tauri
cargo tauri build      # gera AppImage/.deb (Linux) ou .exe/instalador (Windows)
```

## 🗂️ Estrutura

```
lumenlan/
├── assets/logo.png          # imagem-fonte (gera todos os ícones)
├── frontend/                # Svelte + Vite (UI única: desktop e celular)
│   ├── public/              # manifest.webmanifest + ícones PWA
│   └── src/{lib,components} # ws, protocolo, files, chat, QR
└── src-tauri/               # app Tauri + servidor
    └── src/{server,protocol,files,discovery,config}
```

## 📄 Licença

[MIT](LICENSE) © Cauê Araujo
