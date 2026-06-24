import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// Build de saida -> dist/, consumido tanto pela janela nativa do Tauri
// (frontendDist) quanto pelo servidor axum (rust-embed) para os celulares.
export default defineConfig({
  plugins: [svelte()],
  // Tauri espera um host/porta fixos durante o dev.
  clearScreen: false,
  server: {
    host: "0.0.0.0",
    port: 5173,
    strictPort: true,
  },
  build: {
    outDir: "dist",
    emptyOutDir: true,
    // Alvos compativeis com a webview do Tauri.
    target: "es2021",
    sourcemap: false,
  },
});
