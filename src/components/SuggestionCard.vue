<script setup lang="ts">
import { ref } from "vue";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import type { Suggestion } from "../ipc";

// D22 (Plan 5): suggestions shipped show-and-copy only, `edit` (the
// structured, machine-applicable form) deliberately never read, nothing
// ever applied from here. Task 14 (D43, D49) falsifies both: the apply
// button below reads `edit` and emits it, with `config_path`, to
// `BatchView` (this card's parent), which forwards both opaque to
// `apply_suggestion` (Task 14's amended wiring: this card renders and
// emits, it does not orchestrate the load/apply/save round trip itself
// -- `BatchView` owns the picked profile path apply needs, this card
// never sees a file path at all). D22's OWN stated reason for gating
// apply behind the editor -- "one-click apply means comment-preserving
// YAML mutation, machinery that belongs to the editor" -- is dead: D41
// settled that saving rewrites the file canonically and never preserves
// comments, so no comment-preserving machinery exists anywhere to gate
// apply on. The editor+apply pairing survives on a different, stronger
// reason (D41): apply mutates the profile, and whichever surface holds
// the in-memory model owns marking it dirty -- the pairing D41 records
// is plan-scope, not a UI-location or runtime-sharing one (spec 8.2 puts
// apply in the batch view precisely because of this). `config_path` and
// `edit` are the two opaque fields core does all the interpreting of
// (D43): neither is parsed or read here, only forwarded. `yaml_fragment`
// -- the exact text the CLI itself prints for `dry-run-suggestion` -- is
// still shown/copied. `resolves` (the `DiagCode` this fixes) is likewise
// left unshown, matching the CLI's own suggestion header
// (`muxsmith-cli/src/commands/mod.rs`), which never prints it either.
// `applying` (this card's own round trip) drives `aria-busy` alone -- a
// screen reader should only announce busy on the card the user actually
// clicked. `busy` (any batch action in flight, `BatchView`'s own `busy`
// computed) additionally disables the button on EVERY card, not just the
// clicked one: the click handler was already guarded against this
// (`onApplySuggestion`'s `busy.value` early return), but a non-clicked
// card stayed visually enabled during someone else's in-flight apply,
// violating the busy idiom every other action in this view already
// follows.
const props = defineProps<{ suggestion: Suggestion; applying?: boolean; busy?: boolean }>();

const emit = defineEmits<{ apply: [payload: { config_path: string; edit: unknown }] }>();

const copied = ref(false);
let copiedTimeout: ReturnType<typeof setTimeout> | undefined;

async function copy() {
  try {
    await writeText(props.suggestion.yaml_fragment);
    copied.value = true;
    clearTimeout(copiedTimeout);
    // Transient confirmation only; a failed copy leaves the button ready
    // to retry rather than surfacing a modal error over what is a minor,
    // easily-repeated action.
    copiedTimeout = setTimeout(() => {
      copied.value = false;
    }, 2000);
  } catch {
    copied.value = false;
  }
}

/** Emits this card's two opaque fields unmodified; `BatchView` (the
 * parent that owns the picked profile path and the established IPC
 * busy/try/catch/finally idiom) does the actual round trip. Neither
 * field is parsed or interpreted here (D43's binding point). */
function requestApply() {
  emit("apply", { config_path: props.suggestion.config_path, edit: props.suggestion.edit });
}
</script>

<template>
  <article data-testid="batch-suggestion-card">
    <p>
      {{ $t("batch-suggestion-header", { config_path: suggestion.config_path }) }}
    </p>
    <pre><code>{{ suggestion.yaml_fragment }}</code></pre>
    <button
      type="button"
      data-testid="batch-suggestion-copy"
      :title="$ta('batch-suggestion-copy').tooltip"
      @click="copy"
    >
      {{ $t("batch-suggestion-copy") }}
    </button>
    <p
      v-if="copied"
      role="status"
    >
      {{ $t("batch-suggestion-copied") }}
    </p>
    <button
      type="button"
      data-testid="batch-suggestion-apply"
      :title="$ta('batch-suggestion-apply').tooltip"
      :disabled="busy"
      :aria-busy="applying"
      @click="requestApply"
    >
      {{ $t("batch-suggestion-apply") }}
    </button>
  </article>
</template>
