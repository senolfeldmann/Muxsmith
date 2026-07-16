/**
 * Playwright-side helper for the component mount harness (wave-3
 * amendment, mount-harness routing), parallel to `mocks.ts`: injects
 * `mount-harness.js` (built by `vite.mount.config.ts`) into a blank page
 * and drives `window.__muxsmithMount__`/`__muxsmithModel__`/
 * `__muxsmithEmitted__` (typed in `global.d.ts`). No Tauri IPC mock is
 * installed here -- the widgets and, through Task 12, `EditorView` are fed
 * their model as a prop; IPC wiring is Task 13.
 */
import { resolve } from "node:path";
import type { Page } from "@playwright/test";

const MOUNT_HARNESS_PATH = resolve(import.meta.dirname, ".generated/mount-harness.js");

export interface MountSpec {
  component: string;
  props?: Record<string, unknown>;
  locale?: string;
}

export async function mountComponent(page: Page, spec: MountSpec): Promise<void> {
  await page.setContent('<!doctype html><div id="mount"></div>');
  await page.addScriptTag({ path: MOUNT_HARNESS_PATH });
  await page.evaluate((s) => window.__muxsmithMount__(s), spec);
}

export function readModel(page: Page): Promise<unknown> {
  return page.evaluate(() => window.__muxsmithModel__());
}

export function readEmitted(page: Page): Promise<Array<{ event: string; payload: unknown }>> {
  return page.evaluate(() => window.__muxsmithEmitted__);
}
