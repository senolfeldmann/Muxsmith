<script setup lang="ts">
import { useTemplateRef } from "vue";

// D109 decision 6: the confirm surface for the two in-app discard guards
// (`EditorView.vue`'s `pickAndOpen`/`createBlank`, and any second caller
// this component was built for) -- a native `<dialog>` with `showModal()`,
// the same house pattern `SettingsDialog.vue` uses, rather than the dialog
// plugin's own `confirm` (which would need `dialog:allow-message` and is
// only observable at the wire, never as rendered, axe-scannable DOM,
// D109's own rationale). It is its own component, not an inline dialog
// inside its first caller, so a second caller can reuse it -- the four
// string props below are the minimum such a caller needs: the caller
// resolves its own Fluent text and hands it over already translated, so
// this component itself carries no catalog key and the no-raw-text rule
// (D27) is satisfied by binding rather than by an exemption.
defineProps<{
  title: string;
  message: string;
  confirmLabel: string;
  cancelLabel: string;
}>();

const dialogEl = useTemplateRef("dialogEl");

// Set only while a call to `ask()` is outstanding; nulled the moment it is
// settled, so a later event on an already-answered dialog (see `onClose`
// below) is a no-op instead of a second resolution.
let settleAsk: ((confirmed: boolean) => void) | null = null;

/** Opens the dialog and resolves once the user answers. */
function ask(): Promise<boolean> {
  return new Promise((resolve) => {
    settleAsk = resolve;
    dialogEl.value?.showModal();
  });
}

function onConfirm() {
  settleAsk?.(true);
  settleAsk = null;
  dialogEl.value?.close();
}

function onCancel() {
  // No explicit settle here: `.close()` fires the native `close` event
  // below regardless of how the dialog closes, and that handler is the one
  // place `false` is actually resolved -- see its own doc comment.
  dialogEl.value?.close();
}

// The single path that resolves `false`, for every way the dialog can
// close other than the confirm button: the cancel button's own `.close()`
// call above, and Esc -- whose native default action (the `cancel` event,
// left unhandled and therefore not prevented) closes the dialog the same
// way and fires this same `close` event. Esc reading as cancel is the
// safe "do not discard" direction, per D109 decision 6
// (docs/superpowers/specs/2026-07-30-plan-12-decisions.md): "Esc closes
// it, which reads as cancel: the safe direction." A no-op once
// `onConfirm` already settled `true` and nulled `settleAsk`.
function onClose() {
  settleAsk?.(false);
  settleAsk = null;
}

defineExpose({ ask });
</script>

<template>
  <dialog
    ref="dialogEl"
    data-testid="confirm-dialog"
    aria-labelledby="confirm-dialog-title"
    aria-describedby="confirm-dialog-message"
    @close="onClose"
  >
    <h2 id="confirm-dialog-title">
      {{ title }}
    </h2>
    <p id="confirm-dialog-message">
      {{ message }}
    </p>
    <button
      type="button"
      data-testid="confirm-dialog-confirm"
      @click="onConfirm"
    >
      {{ confirmLabel }}
    </button>
    <button
      type="button"
      data-testid="confirm-dialog-cancel"
      @click="onCancel"
    >
      {{ cancelLabel }}
    </button>
  </dialog>
</template>
