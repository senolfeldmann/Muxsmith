/**
 * Shared recent-profiles memory (spec 8.2). The batch view (T10) and the
 * editor (T13c) both feed and render the same `AppSettings.recent_profiles`
 * MRU list. Extracted from BatchView's original `rememberRecentProfile` so
 * the editor can remember a profile on Open through the identical
 * never-clobber round trip -- closing the whole-branch review's Finding 1
 * (spec 8.2 "recent profiles" was batch-only; spec-clause-sweep-at-plan-close).
 */
import { getSettings, setSettings } from "./ipc";
import type { AppSettings } from "./ipc";

/**
 * Mirrors `src-tauri/src/settings.rs::RECENT_PROFILES_CAP` (D27). The Rust
 * side truncates only inside `save()`, so without this client-side cap the
 * rendered MRU list would grow past the limit within one session; capping in
 * the mutation keeps the returned settings identical to what was persisted.
 */
const RECENT_PROFILES_CAP = 10;

/** Pure MRU update: `path` to the front, de-duplicated, capped. */
function withRecentProfile(recents: readonly string[], path: string): string[] {
  return [path, ...recents.filter((p) => p !== path)].slice(0, RECENT_PROFILES_CAP);
}

/**
 * Re-fetches settings, moves `path` to the front of `recent_profiles`
 * (never touching `mkvmerge_path`/`default_jobs`/`locale`/`dir_memory`),
 * persists via `set_settings`, and returns the saved settings. Returns
 * `null` if the round trip fails: a failed recents write must never block the
 * pick/open that triggered it.
 */
export async function rememberRecentProfile(path: string): Promise<AppSettings | null> {
  try {
    const current = await getSettings();
    const next: AppSettings = {
      ...current,
      recent_profiles: withRecentProfile(current.recent_profiles, path),
    };
    await setSettings(next);
    return next;
  } catch (e) {
    console.warn("[recents] failed to persist recent profile:", e);
    return null;
  }
}
