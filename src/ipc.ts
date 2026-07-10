/**
 * Typed IPC wrappers (D23): one thin function per Rust `#[tauri::command]`
 * in `src-tauri/src/lib.rs`/`src-tauri/src/run.rs`, with argument/return
 * types mirroring the Rust structs field-for-field. Deliberately dumb: no
 * retries, no caching, no error translation -- `IpcError` rejects exactly
 * as the shell sends it, and callers render it through their own Fluent
 * bundle (spec 8.4). T9 itself only calls `detectMkvmerge`/`getSettings`/
 * `setSettings`; the rest exist so T10 (`validateProfile`, `dryRun`) and
 * T11 (the run lifecycle, `listRuns`, `getJobLog`) do not each redefine
 * this layer.
 */
import { invoke } from "@tauri-apps/api/core";

/** Mirrors `src-tauri/src/error.rs::IpcError`; the shape every rejected
 * command promise carries. */
export interface IpcError {
  code: string;
  params: Record<string, string>;
}

/** Mirrors `src-tauri/src/settings.rs::DirMemory`. */
export interface DirMemory {
  source?: string;
  output?: string;
}

/** Mirrors `src-tauri/src/settings.rs::AppSettings`. */
export interface AppSettings {
  mkvmerge_path: string | null;
  default_jobs: number;
  locale: string | null;
  recent_profiles: string[];
  dir_memory: Record<string, DirMemory>;
}

/** Mirrors `src-tauri/src/lib.rs::MkvmergeInfo` (D28). */
export interface MkvmergeInfo {
  path: string;
  version: string;
  meets_minimum: boolean;
}

/**
 * A core diagnostic as rendered into a report document (spec 5.2, 7):
 * mirrors `muxsmith_core::report::Diagnostic` plus the `"rendered"` field
 * every `report::json` document function injects (the shell's own
 * passthrough renderer just echoes `code`, never real prose -- the
 * frontend renders `code`/`params` through its own Fluent bundle instead
 * of trusting this field).
 */
export interface Diagnostic {
  code: string;
  severity: "error" | "warning" | "info";
  config_path: string;
  file?: string;
  params: Record<string, string>;
  suggestion_ref?: number;
  rendered: string;
}

/**
 * A `validate_profile`/`dry_run` report document (spec 5.2, 5.5, 7):
 * mirrors `muxsmith_core::report::json::{config_only_document,
 * batch_document}`. `files[].plan`/`suggestions` carry the planner's own
 * `FilePlan`/`Suggestion` shapes, which no T9 code reads and so are left
 * structurally opaque here rather than half-mirrored; T10 (the Batch
 * view, the actual consumer of file-level plan data) is where that shape
 * earns a real type.
 */
export interface ReportDocument {
  config_diagnostics: Diagnostic[];
  files: ReportFile[];
  batch_diagnostics: Diagnostic[];
  suggestions: unknown[];
  mkvmerge_found?: boolean;
}

export interface ReportFile {
  source: string;
  identifier: string;
  plan: unknown | null;
  diagnostics: Diagnostic[];
}

/** Mirrors `src-tauri/src/lib.rs::identify_document`'s JSON shape. */
export interface IdentifyDocument {
  file_name: string;
  identification_format_version: number;
  identifiable: boolean;
  tracks: { id: number; type: string; codec: string }[];
}

/** Mirrors `src-tauri/src/run.rs::StartedRun`. */
export interface StartedRun {
  run_id: string;
  total_jobs: number;
  run_dir: string | null;
}

/** Mirrors `muxsmith_core::executor::job::JobState`. */
export type JobState = "ok" | "warning" | "failed" | "cancelled";

/** Mirrors `muxsmith_core::executor::job::JobOutcome`. */
export interface JobOutcome {
  state: JobState;
  exit_code: number | null;
  warnings: string[];
  errors: string[];
  duration_ms: number;
}

/** One `run_document` job entry: `JobOutcome` plus `index`/`output` (D15). */
export interface RunJobEntry extends JobOutcome {
  index: number;
  output: string;
}

/** A `run`-shaped report document: `ReportDocument` plus `run`'s own D15
 * additions (`report::json::run_document`). */
export interface RunDocument extends ReportDocument {
  jobs: RunJobEntry[];
  summary: { ok: number; warning: number; failed: number; cancelled: number };
}

/** Mirrors `src-tauri/src/run.rs::RunMeta`. */
export interface RunMeta {
  run_id: string;
  started_at: string;
  summary: RunDocument;
}

/** Mirrors `src-tauri/src/run.rs::JoblogStatus`. */
export type JoblogStatus = "complete" | "incomplete" | "unavailable";

/** The `muxsmith://run-finished` window event payload
 * (`run.rs::emit_run_finished`): a `RunDocument` with `joblog_status`
 * spliced in. */
export interface RunFinishedEvent extends RunDocument {
  joblog_status: JoblogStatus;
}

/** Mirrors `muxsmith_core::executor::queue::JobEvent`
 * (`#[serde(tag = "event", rename_all = "snake_case")]`). */
export type JobEvent =
  | { event: "started"; index: number; output: string }
  | { event: "progress"; index: number; percent: number }
  | { event: "warning"; index: number; text: string }
  | { event: "error"; index: number; text: string }
  | { event: "finished"; index: number; outcome: JobOutcome }
  | { event: "output"; index: number; line: string };

/** Window event channel names the run lifecycle emits on (D23); T11 listens
 * on both before invoking `startRun` (`run.rs::start_run`'s documented
 * event-ordering contract). */
export const JOB_EVENT_CHANNEL = "muxsmith://job-event";
export const RUN_FINISHED_CHANNEL = "muxsmith://run-finished";

/** One persisted `job-<index>.json` record (D26), mirrors
 * `executor::joblog::JobRecord`. */
export interface JobLogRecord {
  index: number;
  output: string;
  argv: string[];
  state: JobState;
  exit_code: number | null;
  warnings: string[];
  errors: string[];
  duration_ms: number;
  lines: string[];
  started_at: string | null;
  finished_at: string;
}

// --- read-only + settings commands (T7) -------------------------------

export function validateProfile(path: string): Promise<ReportDocument> {
  return invoke<ReportDocument>("validate_profile", { path });
}

export function dryRun(
  profile: string,
  source?: string,
  output?: string,
): Promise<ReportDocument> {
  return invoke<ReportDocument>("dry_run", { profile, source, output });
}

export function identify(file: string): Promise<IdentifyDocument> {
  return invoke<IdentifyDocument>("identify", { file });
}

export function detectMkvmerge(): Promise<MkvmergeInfo> {
  return invoke<MkvmergeInfo>("detect_mkvmerge");
}

export function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>("get_settings");
}

export function setSettings(settings: AppSettings): Promise<void> {
  return invoke<void>("set_settings", { settings });
}

// --- run lifecycle commands (T8) ---------------------------------------

export function startRun(params: {
  profile: string;
  source?: string;
  output?: string;
  jobs?: number;
}): Promise<StartedRun> {
  return invoke<StartedRun>("start_run", params);
}

export function cancelRun(): Promise<void> {
  return invoke<void>("cancel_run");
}

export function cancelJob(index: number): Promise<void> {
  return invoke<void>("cancel_job", { index });
}

export function listRuns(): Promise<RunMeta[]> {
  return invoke<RunMeta[]>("list_runs");
}

export function getJobLog(runId: string, index: number): Promise<JobLogRecord> {
  return invoke<JobLogRecord>("get_job_log", { runId, index });
}
