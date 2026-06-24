# Imagem-fonte do LumenLan

Coloque **uma única** imagem de logo aqui. Todos os ícones (desktop e PWA) são
gerados a partir dela — nada de ícone gerado entra no git.

## O que enviar

| Item | Recomendado |
|------|-------------|
| Arquivo | `logo.png` (e, se tiver, `logo.svg` como fonte vetorial) |
| Dimensões | **quadrada, ≥ 1024×1024 px** |
| Formato | PNG com transparência (RGBA / 32-bit) |
| Conteúdo | sem cantos arredondados nem sombra — o SO aplica a máscara |
| Margem | deixe ~12–15% de respiro nas bordas (ajuda no ícone "maskable" do Android) |

Uma imagem 1024×1024 cobre tudo. SVG é bem-vindo como fonte extra (escala perfeita).

## O que é gerado a partir dela

**Desktop (Tauri — Linux + Windows)** via `cargo tauri icon assets/logo.png`:
- Linux: `32x32.png`, `128x128.png`, `128x128@2x.png`, `icon.png`
- Windows: `icon.ico`

**PWA (Android + iOS pelo navegador)** — referenciados pelo `manifest.webmanifest`:
- `icon-192.png` (192×192)
- `icon-512.png` (512×512)
- `icon-512-maskable.png` (512×512, com respiro — ícone adaptativo do Android)
- `apple-touch-icon.png` (180×180 — tela inicial do iOS)
- `favicon.png` (32×32)

Depois de colocar a imagem aqui, é só avisar que eu regenero tudo.
