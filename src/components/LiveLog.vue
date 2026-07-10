<script setup lang="ts">
// Live output pane (T11 Step 2, D29 "the live log pane is role=log"): fed
// by `job-event`'s `output` variant (verbatim mkvmerge lines, tags
// included -- D24), DOM-capped by the caller at 5000 entries (the full log
// is always in the persisted job file, D26/D30 -- this pane is a live tail,
// not the record of truth). Auto-scrolls to the newest line unless the
// user has scrolled up to read earlier output.
import { computed, nextTick, ref, watch } from "vue";
import type { JobRowData } from "../jobRowState";

interface LogLine {
  index: number;
  line: string;
}

const props = defineProps<{
  jobs: JobRowData[];
  lines: LogLine[];
}>();

const selected = ref<number | "all">("all");

// A new run must not inherit the previous run's per-job filter (its index
// may not even exist in the new batch, silently blanking the pane).
// JobsView replaces its `jobs` array reference exactly once per run
// dispatch (`jobs.value = []`) and only mutates it afterwards, so a
// non-deep watch on the prop reference is a precise new-run signal -- no
// extra prop or event needed.
watch(
  () => props.jobs,
  () => {
    selected.value = "all";
  },
);

const filtered = computed(() =>
  selected.value === "all" ? props.lines : props.lines.filter((l) => l.index === selected.value),
);

const logEl = ref<HTMLElement | null>(null);
// Sticky-bottom auto-scroll: true whenever the pane is scrolled at (or
// near) its bottom edge, so a fresh line keeps the view pinned there; a
// user who scrolls up to read history clears it, and new lines then land
// without yanking the view away from what they are reading.
const stickToBottom = ref(true);

function onScroll() {
  const el = logEl.value;
  if (!el) {
    return;
  }
  stickToBottom.value = el.scrollHeight - el.scrollTop - el.clientHeight < 16;
}

watch(filtered, async () => {
  if (!stickToBottom.value) {
    return;
  }
  await nextTick();
  const el = logEl.value;
  if (el) {
    el.scrollTop = el.scrollHeight;
  }
});
</script>

<template>
  <div data-testid="live-log-panel">
    <label for="live-log-filter">{{ $t("jobs-log-filter-label") }}</label>
    <select
      id="live-log-filter"
      v-model="selected"
      data-testid="live-log-filter"
    >
      <option value="all">
        {{ $t("jobs-log-filter-all") }}
      </option>
      <option
        v-for="job in jobs"
        :key="job.index"
        :value="job.index"
      >
        {{ job.output ?? $t("jobs-row-output-pending", { index: job.index }) }}
      </option>
    </select>
    <div
      ref="logEl"
      role="log"
      :aria-label="$t('jobs-log-region-label')"
      data-testid="live-log"
      class="live-log"
      @scroll="onScroll"
    >
      <div
        v-for="(entry, i) in filtered"
        :key="i"
      >
        {{ entry.line }}
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Functionally required, not decorative: role="log" auto-scroll only means
   something if the pane is a bounded, scrollable box. */
.live-log {
  max-height: 16rem;
  overflow-y: auto;
}
</style>
