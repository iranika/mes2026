<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { highlightMes } from "../highlightMes";

const model = defineModel<string>({ required: true });

const textareaRef = ref<HTMLTextAreaElement | null>(null);
const backdropRef = ref<HTMLPreElement | null>(null);

const highlighted = computed(() => highlightMes(model.value));

function syncScroll() {
  const ta = textareaRef.value;
  const bd = backdropRef.value;
  if (!ta || !bd) return;
  bd.scrollTop = ta.scrollTop;
  bd.scrollLeft = ta.scrollLeft;
}

watch(model, () => {
  requestAnimationFrame(syncScroll);
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
      @scroll="syncScroll"
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

@media (max-width: 900px) {
  .mes-editor {
    min-height: 35vh;
  }
}
</style>
