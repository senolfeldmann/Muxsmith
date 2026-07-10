import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

// Pre-test build step, chained into the `test:e2e` script (NOT part of the
// app's own `pnpm build`/`tauri.conf.json` bundling -- this output never
// ships in the Tauri binary). Bundles `tauri-mock-entry.ts` into one
// dependency-free IIFE so `mocks.ts` can inject the REAL
// `@tauri-apps/api/mocks`/`event` code into a plain browser page via
// `page.addInitScript({ path })`, which reads the file straight off disk
// (no HTTP serving involved -- this never touches `dist/`/`public/`, so it
// can never leak into the shipped Tauri bundle). Output is gitignored and
// rebuilt on every `test:e2e` run, so it can never drift from the pinned
// `@tauri-apps/api` version `src/` itself uses.
const here = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  build: {
    outDir: resolve(here, ".generated"),
    emptyOutDir: true,
    minify: false,
    lib: {
      entry: resolve(here, "tauri-mock-entry.ts"),
      name: "MuxsmithE2EHarness",
      formats: ["iife"],
      fileName: () => "tauri-mock-harness.js",
    },
  },
});
