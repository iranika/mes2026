<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { highlightMes } from "../highlightMes";

const model = defineModel<string>({ required: true });
const emit = defineEmits<{
  scrollRatio: [ratio: number];
}>();

const textareaRef = ref<HTMLTextAreaElement | null>(null);
const backdropRef = ref<HTMLPreElement | null>(null);
let suppressScrollEmit = false;

const highlighted = computed(() => highlightMes(model.value));

function scrollMetrics(el: HTMLElement) {
  const max = el.scrollHeight - el.clientHeight;
  return { max, ratio: max <= 0 ? 0 : el.scrollTop / max };
}

function syncBackdrop() {
  const ta = textareaRef.value;
  const bd = backdropRef.value;
  if (!ta || !bd) return;
  bd.scrollTop = ta.scrollTop;
  bd.scrollLeft = ta.scrollLeft;
}

function onTextareaScroll() {
  syncBackdrop();
  if (suppressScrollEmit) return;
  const ta = textareaRef.value;
  if (!ta) return;
  emit("scrollRatio", scrollMetrics(ta).ratio);
}

/** Apply a proportional scroll position from the paired preview pane. */
function setScrollRatio(ratio: number) {
  const ta = textareaRef.value;
  if (!ta) return;
  const { max } = scrollMetrics(ta);
  const next = Math.max(0, Math.min(1, ratio)) * max;
  if (Math.abs(ta.scrollTop - next) < 1) return;
  suppressScrollEmit = true;
  ta.scrollTop = next;
  syncBackdrop();
  requestAnimationFrame(() => {
    suppressScrollEmit = false;
  });
}

defineExpose({ setScrollRatio });

watch(model, () => {
  requestAnimationFrame(syncBackdrop);
});
</script>

<template>
  <div class="mes-editor">
    <pre
      ref="backdropRef"
      class="backdrop"
      aria-hidden="true"
      v-html="highlighted"
    /><textarea
      ref="textareaRef"
      v-model="model"
      class="input"
      spellcheck="false"
      aria-label="MeS source"
      @scroll="onTextareaScroll"
    />
  </div>
</template>

<style scoped>
.mes-editor {
  position: relative;
  flex: 1;
  min-height: 55vh;
  border: 1px solid #cbd5e1;
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.9);
  overflow: hidden;
}

.backdrop,
.input {
  margin: 0;
  position: absolute;
  inset: 0;
  box-sizing: border-box;
  padding: 0.75rem;
  font-family: "IBM Plex Mono", ui-monospace, monospace;
  font-size: 0.9rem;
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-word;
  overflow: auto;
  border: 0;
  tab-size: 2;
}

.backdrop {
  pointer-events: none;
  color: #0f172a;
  background: transparent;
}

.input {
  resize: none;
  color: transparent;
  caret-color: #0f172a;
  background: transparent;
  outline: none;
}

.input::selection {
  background: rgba(37, 99, 235, 0.25);
  color: transparent;
}

.backdrop :deep(.tok-prefix) {
  font-weight: 700;
}

.backdrop :deep(.tok-char) {
  color: #1d4ed8;
}

.backdrop :deep(.tok-comment) {
  color: #64748b;
  font-style: italic;
}

.backdrop :deep(.tok-sound) {
  color: #b45309;
}

.backdrop :deep(.tok-pos) {
  color: #be185d;
}

.backdrop :deep(.tok-timing) {
  color: #0f766e;
}

.backdrop :deep(.tok-delimiter) {
  color: #7c3aed;
  font-weight: 600;
}

@media (forced-colors: active) {
  .backdrop {
    display: none;
  }

  .input {
    color: CanvasText;
    caret-color: CanvasText;
    background: Canvas;
  }

  .input::selection {
    color: HighlightText;
    background: Highlight;
  }
}

@media (max-width: 900px) {
  .mes-editor {
    min-height: 35vh;
  }
}
</style>
