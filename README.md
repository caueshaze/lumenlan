<div align="center">

<img src="assets/logo.png" alt="LumenLan" width="128" height="128" />

# LumenLan

**Chat e compartilhamento de imagens na sua rede local — sem internet, sem cadastro.**

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
- 🖼️ **Compartilhamento de imagens** em chunks, salvas automaticamente em `./lumenlan_received/`.
- 📱 **PWA**: a interface web é instalável no celular (ícone na tela inicial).
- 🔌 **Binário único self-contained**: o frontend vai embutido no executável.
- 🛰️ **Descoberta automática** via mDNS (`lumenlan.local`) + **QR code** na janela nativa.
- ⚡ **Assíncrono** (Tokio): imagens pesadas não travam o chat de texto.

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
WebSocket. O servidor `axum` (embutido) faz o broadcast 1 → N e grava as imagens
recebidas em disco via streaming.

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

Com o app aberto, clique em **📱 Conectar** para ver o **QR code** e o endereço.
No celular (mesma Wi-Fi), escaneie ou abra:

```
http://<ip-do-desktop>:8787      # ex.: http://192.168.15.5:8787
http://lumenlan.local:8787       # em redes com suporte a mDNS
```

> **PWA:** no iOS use *Adicionar à Tela de Início* (Safari). No Android, a
> instalação completa de PWA exige HTTPS; em HTTP na LAN funciona como atalho.

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
