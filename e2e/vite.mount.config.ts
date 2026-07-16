import { resolve } from "node:path";
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

const here = import.meta.dirname;

export default defineConfig({
  plugins: [vue()],
  // Vue's `esm-bundler` build deliberately leaves `process.env.NODE_ENV`
  // unreplaced (dev-mode warning guards) and expects the consuming
  // bundler to define it -- `vite build` on the app's own `index.html`
  // does this automatically via its internal esbuild `define` wiring, but
  // `build.lib` (this config, and the one it is parallel to,
  // `vite.harness.config.ts`) does not get the same automatic
  // substitution for a bundled dependency (a known Vite library-mode gap,
  // not exercised until this config: `vite.harness.config.ts` bundles
  // only `@tauri-apps/api`, which has no `process.env` reference).
  // Confirmed empirically: without this, the IIFE throws `ReferenceError:
  // process is not defined` at the very first Vue module evaluated
  // (`shared.esm-bundler.js`'s `EMPTY_OBJ` constant), before
  // `window.__muxsmithMount__` is ever assigned.
  define: {
    "process.env.NODE_ENV": JSON.stringify("production"),
  },
  build: {
    outDir: resolve(here, ".generated"),
    emptyOutDir: false, // must NOT wipe tauri-mock-harness.js, built by the step before
    minify: false,
    lib: {
      entry: resolve(here, "mount-entry.ts"),
      name: "MuxsmithMountHarness",
      formats: ["iife"],
      fileName: () => "mount-harness.js",
    },
  },
});
