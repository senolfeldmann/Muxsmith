import { computed, inject, type ComputedRef, type InjectionKey } from "vue";
import type { Diagnostic } from "../ipc";

// D57: the mapping from `Diagnostic.config_path` to a control is EXACT
// string equality against the paths the widget tree constructs while
// rendering, mirroring core's emission grammar verbatim (design section 1).
// No parser exists on either side: core's paths are opaque keys, the
// frontend builds the same keys by construction (path threading in the
// widgets) and looks them up here. A path with no rendered control
// (`profile_version`, any future core path the tree does not know) is
// silently panel-only -- the diagnostics panel is never filtered.
export const editorDiagnosticsByPath: InjectionKey<
  ComputedRef<Map<string, Diagnostic[]>>
> = Symbol("editorDiagnosticsByPath");

const SEVERITY_ORDER = ["error", "warning", "info"] as const;
export type Severity = (typeof SEVERITY_ORDER)[number];

/** The worst severity across `diags` (error > warning > info -- the three
 *  severities are the complete set, `report/mod.rs`), or `null` for an empty
 *  list. Shared so `useDiagAnchor` and the per-row anchors
 *  (`PropertyMapWidget`, the bespoke rule grid) compute severity one way. */
export function worstSeverity(diags: Diagnostic[]): Severity | null {
  let worst: Severity | null = null;
  for (const d of diags) {
    const s = d.severity as Severity;
    if (worst === null || SEVERITY_ORDER.indexOf(s) < SEVERITY_ORDER.indexOf(worst)) {
      worst = s;
    }
  }
  return worst;
}

/** Anchor state for one widget path: its diagnostics and worst severity,
 *  reactive against the injected map. Exact string equality against core's
 *  emitted paths; no parsing, no normalization (D57). `path` is a getter so
 *  a widget with per-item paths can drive it, and `inject`'s `undefined`
 *  default means a widget mounted outside an `EditorView` (the standalone
 *  mount harness) simply renders no marker. */
export function useDiagAnchor(path: () => string | undefined) {
  const byPath = inject(editorDiagnosticsByPath, undefined);
  const diags = computed<Diagnostic[]>(() => {
    const p = path();
    if (p === undefined || byPath === undefined) return [];
    return byPath.value.get(p) ?? [];
  });
  const severity = computed<Severity | null>(() => worstSeverity(diags.value));
  return { diags, severity };
}
