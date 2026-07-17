<script setup lang="ts">
import { ref, watch } from "vue";
import MesEditor from "./components/MesEditor.vue";
import {
  convertMes,
  getMesBackend,
  type MesBackend,
  type PreviewMode,
} from "./mesApi";
import {
  exportPreview,
  openMesFile,
  saveMesFile,
  saveMesFileAs,
} from "./fileIo";

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
const filePath = ref<string | null>(null);
const result = ref<string>("");
const error = ref<string>("");
const mode = ref<PreviewMode>("json");
const converting = ref(false);
const ioBusy = ref(false);
const status = ref<string>("");
const backend = ref<MesBackend>(getMesBackend());

let debounceTimer: ReturnType<typeof setTimeout> | null = null;

async function convert() {
  error.value = "";
  converting.value = true;
  backend.value = getMesBackend();
  try {
    result.value = await convertMes(mode.value, mesText.value);
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
  filePath.value = null;
  status.value = "サンプルを読み込みました";
}

async function onOpen() {
  ioBusy.value = true;
  status.value = "";
  try {
    const opened = await openMesFile();
    if (!opened) return;
    mesText.value = opened.contents;
    filePath.value = opened.path;
    status.value = opened.path ? `開きました: ${opened.path}` : "ファイルを開きました";
  } catch (e) {
    error.value = String(e);
  } finally {
    ioBusy.value = false;
  }
}

async function onSave() {
  ioBusy.value = true;
  status.value = "";
  try {
    const path = await saveMesFile(mesText.value, filePath.value);
    if (!path) return;
    filePath.value = path;
    status.value = `保存しました: ${path}`;
  } catch (e) {
    error.value = String(e);
  } finally {
    ioBusy.value = false;
  }
}

async function onSaveAs() {
  ioBusy.value = true;
  status.value = "";
  try {
    const path = await saveMesFileAs(mesText.value);
    if (!path) return;
    filePath.value = path;
    status.value = `保存しました: ${path}`;
  } catch (e) {
    error.value = String(e);
  } finally {
    ioBusy.value = false;
  }
}

async function onExport() {
  if (!result.value) {
    status.value = "エクスポートするプレビューがありません";
    return;
  }
  ioBusy.value = true;
  status.value = "";
  try {
    const ok = await exportPreview(mode.value, result.value);
    if (ok) status.value = `${mode.value.toUpperCase()} をエクスポートしました`;
  } catch (e) {
    error.value = String(e);
  } finally {
    ioBusy.value = false;
  }
}
</script>

<template>
  <main class="container">
    <header class="header">
      <div class="title-row">
        <h1>MeS Editor</h1>
        <span class="backend" :data-backend="backend">
          {{ backend === "tauri" ? "Tauri" : "Browser / WASM" }}
        </span>
      </div>
      <p class="subtitle">MeS を Medo / VTT / ワードカウント / チャット形式へ変換</p>
      <div class="toolbar">
        <button type="button" class="ghost" :disabled="ioBusy" @click="onOpen">開く</button>
        <button type="button" class="ghost" :disabled="ioBusy" @click="onSave">保存</button>
        <button type="button" class="ghost" :disabled="ioBusy" @click="onSaveAs">別名で保存</button>
        <button type="button" class="ghost" :disabled="ioBusy || !result" @click="onExport">
          プレビューを書き出し
        </button>
        <button type="button" class="ghost" @click="resetSample">サンプルに戻す</button>
        <span v-if="filePath" class="path" :title="filePath">{{ filePath }}</span>
      </div>
      <p v-if="status" class="status">{{ status }}</p>
    </header>

    <div class="editor-row">
      <section class="pane">
        <div class="pane-head">
          <h2>MeS 入力</h2>
          <span class="hint">@ # $ ! &amp; をハイライト</span>
        </div>
        <MesEditor v-model="mesText" />
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

.title-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex-wrap: wrap;
}

.header h1 {
  margin: 0;
  font-family: "IBM Plex Serif", Georgia, serif;
  font-size: 1.75rem;
  letter-spacing: 0.02em;
}

.backend {
  font-size: 0.75rem;
  font-weight: 600;
  letter-spacing: 0.04em;
  text-transform: uppercase;
  color: #1e3a5f;
  border: 1px solid #93c5fd;
  background: rgba(219, 234, 254, 0.7);
  padding: 0.2rem 0.5rem;
  border-radius: 4px;
}

.backend[data-backend="tauri"] {
  color: #14532d;
  border-color: #86efac;
  background: rgba(220, 252, 231, 0.75);
}

.subtitle {
  margin: 0.35rem 0 0.75rem;
  color: #475569;
  font-size: 0.95rem;
}

.toolbar {
  display: flex;
  flex-wrap: wrap;
  gap: 0.4rem;
  align-items: center;
}

.path {
  margin-left: 0.25rem;
  max-width: 28rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 0.8rem;
  color: #64748b;
  font-family: "IBM Plex Mono", ui-monospace, monospace;
}

.status {
  margin: 0.5rem 0 0;
  font-size: 0.85rem;
  color: #334155;
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

.hint {
  font-size: 0.75rem;
  color: #64748b;
}

.output {
  min-height: 55vh;
  width: 100%;
  box-sizing: border-box;
  border: 1px solid #1e293b;
  border-radius: 8px;
  padding: 0.75rem;
  font-family: "IBM Plex Mono", ui-monospace, monospace;
  font-size: 0.9rem;
  line-height: 1.5;
  margin: 0;
  overflow: auto;
  background: #0f172a;
  color: #e2e8f0;
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

  .output {
    min-height: 35vh;
  }
}
</style>
