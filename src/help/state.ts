import { ref } from "vue";

/** Help-mode app-shell state (D52). */
export const helpMode = ref(false);
export const pinnedId = ref<string | null>(null);
export const hoverId = ref<string | null>(null);

/** Default sidebar topic per view (D52/E2: the three spec-8.2 views only). */
export const VIEW_TOPICS = {
  batch: "view-batch",
  jobs: "view-jobs",
  editor: "view-editor",
} as const;
