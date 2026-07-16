<script setup lang="ts">
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";

type PreviewMode = "json" | "vtt" | "count" | "chat";

const SAMPLE = `title: demo
----
@Alice
こんにちは
# 導入セリフ
$ chime

@Bob
&00:00:01.000 --> 00:00:03.000
やあ、元気？

Alice「フラット記法の発話です」
`;

const mesText = ref<string>(SAMPLE);
const result = ref<string>("");
const error = ref<string>("");
const mode = ref<PreviewMode>("json");
const converting = ref(false);

let debounceTimer: ReturnType<typeof setTimeout> | null = null;

const commands: Record<PreviewMode, string> = {
  json: "mes_to_medo",
  vtt: "mes_to_vtt",
  count: "mes_word_count",
  chat: "mes_to_chat",
};

async function convert() {
  error.value = "";
  converting.value = true;
  try {
    result.value = (await invoke(commands[mode.value], {
      text: mesText.value,
    })) as string;
  } catch (e) {
    error.value = String(e);
    result.value = "";
  } finally {
    converting.value = false;
  }
}

function scheduleConvert() {
  if (debounceTimer) clearTimeout(debounceTimer);
  debounceTimer = setTimeout(() => {
    void convert();
  }, 250);
}

watch([mesText, mode], scheduleConvert, { immediate: true });

function resetSample() {
  mesText.value = SAMPLE;
}
</script>

<template>
  <main class="container">
    <header class="header">
      <h1>MeS Editor</h1>
      <p class="subtitle">MeS を Medo / VTT / ワードカウント / チャット形式へ変換</p>
    </header>

    <div class="editor-row">
      <section class="pane">
        <div class="pane-head">
          <h2>MeS 入力</h2>
          <button type="button" class="ghost" @click="resetSample">サンプルに戻す</button>
        </div>
        <textarea
          v-model="mesText"
          class="editor"
          spellcheck="false"
          aria-label="MeS source"
        />
        <div class="controls">
          <button type="button" @click="convert" :disabled="converting">
            {{ converting ? "変換中…" : "再変換" }}
          </button>
        </div>
      </section>

      <section class="pane preview">
        <div class="pane-head">
          <h2>プレビュー</h2>
          <div class="tabs" role="tablist">
            <button
              type="button"
              role="tab"
              :aria-selected="mode === 'json'"
              :class="{ active: mode === 'json' }"
              @click="mode = 'json'"
            >
              JSON
            </button>
            <button
              type="button"
              role="tab"
              :aria-selected="mode === 'vtt'"
              :class="{ active: mode === 'vtt' }"
              @click="mode = 'vtt'"
            >
              VTT
            </button>
            <button
              type="button"
              role="tab"
              :aria-selected="mode === 'count'"
              :class="{ active: mode === 'count' }"
              @click="mode = 'count'"
            >
              Count
            </button>
            <button
              type="button"
              role="tab"
              :aria-selected="mode === 'chat'"
              :class="{ active: mode === 'chat' }"
              @click="mode = 'chat'"
            >
              Chat
            </button>
          </div>
        </div>
        <div v-if="error" class="error">{{ error }}</div>
        <pre v-if="mode !== 'chat'" class="output">{{ result }}</pre>
        <div v-else class="output chat" v-html="result"></div>
      </section>
    </div>
  </main>
</template>

<style scoped>
:root {
  color-scheme: light;
}

.container {
  min-height: 100vh;
  padding: 1.25rem 1.5rem 2rem;
  background:
    radial-gradient(circle at top left, #dbeafe 0%, transparent 40%),
    linear-gradient(160deg, #f8fafc 0%, #eef2ff 55%, #f1f5f9 100%);
  color: #0f172a;
  font-family: "IBM Plex Sans", "Segoe UI", sans-serif;
}

.header {
  margin-bottom: 1rem;
}

.header h1 {
  margin: 0;
  font-family: "IBM Plex Serif", Georgia, serif;
  font-size: 1.75rem;
  letter-spacing: 0.02em;
}

.subtitle {
  margin: 0.35rem 0 0;
  color: #475569;
  font-size: 0.95rem;
}

.editor-row {
  display: flex;
  gap: 1rem;
  align-items: stretch;
}

.pane {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.pane-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  flex-wrap: wrap;
}

.pane-head h2 {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
}

.editor,
.output {
  min-height: 55vh;
  width: 100%;
  box-sizing: border-box;
  border: 1px solid #cbd5e1;
  border-radius: 8px;
  padding: 0.75rem;
  font-family: "IBM Plex Mono", ui-monospace, monospace;
  font-size: 0.9rem;
  line-height: 1.5;
  background: rgba(255, 255, 255, 0.85);
}

.editor {
  resize: vertical;
}

.output {
  margin: 0;
  overflow: auto;
  background: #0f172a;
  color: #e2e8f0;
  border-color: #1e293b;
  white-space: pre-wrap;
  word-break: break-word;
}

.output.chat {
  font-family: "IBM Plex Sans", "Segoe UI", sans-serif;
  line-height: 1.7;
}

.controls {
  display: flex;
  gap: 0.5rem;
}

.tabs {
  display: flex;
  gap: 0.25rem;
}

button {
  padding: 0.4rem 0.85rem;
  border-radius: 6px;
  border: 1px solid #94a3b8;
  background: #1d4ed8;
  color: white;
  cursor: pointer;
  font: inherit;
}

button:disabled {
  opacity: 0.6;
  cursor: wait;
}

button.ghost,
.tabs button {
  background: transparent;
  color: #334155;
}

.tabs button.active {
  background: #1e293b;
  color: #f8fafc;
  border-color: #1e293b;
}

.error {
  color: #b91c1c;
  background: #fef2f2;
  border: 1px solid #fecaca;
  border-radius: 6px;
  padding: 0.5rem 0.75rem;
}

@media (max-width: 900px) {
  .editor-row {
    flex-direction: column;
  }

  .editor,
  .output {
    min-height: 35vh;
  }
}
</style>
