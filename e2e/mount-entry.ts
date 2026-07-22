/**
 * Browser-side component mount harness (wave-3 amendment, mount-harness
 * routing): bundled standalone by `vite.mount.config.ts` into a
 * dependency-free IIFE (`e2e/.generated/mount-harness.js`), injected into a
 * blank `page.setContent()` page via `page.addScriptTag({ path })`
 * (`mount.ts`). Extends the `tauri-mock-entry.ts` precedent (real
 * dependencies bundled once, re-exported on a `window.__muxsmith*__`
 * global) to solve a different problem: no editor mount point exists in
 * the running app before Task 13 (`src/main.ts` mounts only `App.vue`,
 * whose `View` union is `"batch" | "jobs"`), so Tasks 10-12's rendering
 * assertions cannot reach editor UI through `page.goto("/")`. This module
 * mounts one component at a time in isolation instead.
 *
 * `import.meta.glob`'s result is PATH-keyed (`"../src/editor/widgets/
 * TextWidget.vue"`, not `"TextWidget"`) -- `resolvePath` below
 * reconstructs the full path from the bare basename a caller passes, there
 * is no separate basename-to-module map. `eager: true` is mandatory: an
 * IIFE build forbids code-splitting, which a lazy glob would introduce.
 */
import { createApp, h, ref } from "vue";
import type { Component } from "vue";
import { createFluentVue } from "fluent-vue";
import { buildBundles } from "../src/i18n";
import { topicHtml } from "../src/help/topics";

const modules = import.meta.glob<{ default: Component }>(
  ["../src/editor/widgets/*.vue", "../src/views/EditorView.vue"],
  { eager: true },
);

function resolvePath(component: string): string {
  return component === "EditorView"
    ? "../src/views/EditorView.vue"
    : `../src/editor/widgets/${component}.vue`;
}

interface MountSpec {
  component: string;
  props?: Record<string, unknown>;
  locale?: string;
}

let currentApp: ReturnType<typeof createApp> | null = null;

function mount(spec: MountSpec): void {
  currentApp?.unmount();
  window.__muxsmithEmitted__ = [];

  const path = resolvePath(spec.component);
  const mod = modules[path];
  if (!mod) {
    throw new Error(`unknown mount component "${spec.component}"`);
  }
  const Comp = mod.default;

  const model = ref(spec.props?.modelValue);
  window.__muxsmithModel__ = () => model.value;

  const app = createApp({
    render: () =>
      h(Comp, {
        ...spec.props,
        modelValue: model.value,
        "onUpdate:modelValue": (v: unknown) => {
          model.value = v;
          window.__muxsmithEmitted__.push({ event: "update:modelValue", payload: v });
        },
      }),
  });
  app.use(createFluentVue({ bundles: buildBundles(spec.locale ?? "en") }));

  const root = document.getElementById("mount");
  if (!root) {
    throw new Error('mount-entry: no "#mount" element in the page');
  }
  currentApp = app;
  app.mount(root);
}

window.__muxsmithMount__ = mount;
window.__muxsmithTopicHtml__ = topicHtml;
