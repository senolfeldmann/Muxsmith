<script setup lang="ts">
import { ref } from "vue";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import type { Suggestion } from "../ipc";

// D22: Plan 5 ships suggestions as show-and-copy only, never applied --
// the profile is never mutated from here. `edit` (the structured,
// machine-applicable form) is deliberately never read; only
// `config_path` and `yaml_fragment` -- the exact text the CLI itself
// prints for `dry-run-suggestion` -- are shown/copied. `resolves` (the
// `DiagCode` this fixes) is likewise left unshown, matching the CLI's own
// suggestion header (`muxsmith-cli/src/commands/mod.rs`), which never
// prints it either.
const props = defineProps<{ suggestion: Suggestion }>();

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
      :title="$t('batch-suggestion-copy-tooltip')"
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
  </article>
</template>
