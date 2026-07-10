import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// Tauri's dev server contract (https://v2.tauri.app/start/frontend/vite/):
// fixed port, fail instead of falling back to another one (Tauri's
// `devUrl` in tauri.conf.json is not renegotiated), and ignore src-tauri/
// so a Rust rebuild does not also retrigger the Vite dev server.
export default defineConfig({
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});
