<script setup lang="ts">
import { computed } from "vue";
import { topicHtml } from "../help/topics";
import { currentLocale } from "../i18n/fluent";

const props = defineProps<{ topicId: string }>();
const html = computed(() => topicHtml(props.topicId, currentLocale.value));
</script>

<template>
  <!-- eslint-disable vue/no-v-html -- D50's single licensed v-html site:
       first-party, repo-reviewed markdown only. -->
  <aside
    data-testid="help-sidebar"
    :aria-label="$t('help-sidebar-label')"
    v-html="html"
  />
  <!-- eslint-enable vue/no-v-html -->
</template>
