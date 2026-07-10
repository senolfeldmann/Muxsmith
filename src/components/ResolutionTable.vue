<script setup lang="ts">
import DiagnosticsPanel from "./DiagnosticsPanel.vue";
import type { PlanAssignment, ReportFile } from "../ipc";

// One `batch_document.files[]` entry (spec 5, T10 brief step 2): the
// rule -> resolved track table plus this file's own diagnostics.
// `file.plan` is `null` exactly when the file carries an error-severity
// diagnostic (spec 5.1, no plan produced); the table only ever renders
// when a plan exists, matching what `plan.assignments` can express -- an
// empty table body would misrepresent "planning never reached this file"
// as "this file resolves to zero tracks".
//
// Unmatched `track_id === null` renders as the ASCII "-" placeholder, not
// a Fluent key: it mirrors the CLI's own identical unlocalized "-"
// (`muxsmith-cli/src/commands/mod.rs::print_batch_human`) for the same
// value, spec 7's "CLI and GUI render the same report structures"
// consistency. `track_kind` (video/audio/subtitles/buttons) is likewise
// left as-is: an mkvmerge-identification vocabulary word passed through
// from core, not app-authored UI copy (spec 8.4's `detail`-param
// exception covers the same kind of third-party passthrough text).
defineProps<{ file: ReportFile }>();

function resolvedTrackLabel(a: PlanAssignment): string {
  return a.track_id === null ? "-" : `${a.track_id} (${a.track_kind})`;
}
</script>

<template>
  <section
    data-testid="batch-resolution-table"
    :data-index="file.identifier"
  >
    <table v-if="file.plan">
      <caption>
        {{
          $t("batch-file-caption", {
            source: file.source,
            identifier: file.identifier,
            output: file.plan.output,
          })
        }}
      </caption>
      <thead>
        <tr>
          <th scope="col">
            {{ $t("batch-resolution-rule-header") }}
          </th>
          <th scope="col">
            {{ $t("batch-resolution-track-header") }}
          </th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="a in file.plan.assignments"
          :key="a.rule_index"
        >
          <td>{{ a.rule_index }}</td>
          <td>{{ resolvedTrackLabel(a) }}</td>
        </tr>
      </tbody>
    </table>
    <p v-else>
      {{ $t("batch-file-no-plan", { source: file.source, identifier: file.identifier }) }}
    </p>
    <DiagnosticsPanel :diagnostics="file.diagnostics" />
  </section>
</template>
