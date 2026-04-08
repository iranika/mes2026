<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const mesText = ref<string>("----\n# header\n----\n@Alice\nこんにちは\n\n@Bob\nやあ\n");
const result = ref<string>("");
const error = ref<string>("");

async function convert() {
  error.value = "";
  try {
    result.value = await invoke("mes_to_medo", { text: mesText.value }) as string;
  } catch (e) {
    error.value = String(e);
  }
}

// convert on first load
convert();
</script>

<template>
  <main class="container">
    <h1>MeS → Medo JSON プレビュー</h1>

    <div class="editor-row">
      <section class="pane">
        <h2>MeS 入力</h2>
        <textarea v-model="mesText" class="editor" />
        <div class="controls">
          <button @click="convert">変換</button>
        </div>
      </section>

      <section class="pane preview">
        <h2>Medo JSON プレビュー</h2>
        <div v-if="error" class="error">{{ error }}</div>
        <pre class="output">{{ result }}</pre>
      </section>
    </div>
  </main>
</template>

<style scoped>
.container { padding: 1rem; }
.editor-row { display: flex; gap: 1rem; }
.pane { flex: 1; display: flex; flex-direction: column; }
.editor { min-height: 40vh; width: 100%; font-family: monospace; padding: 0.5rem; }
.controls { margin-top: 0.5rem; }
.output { background: #0f1724; color: #e6eef8; padding: 1rem; overflow: auto; min-height: 40vh; }
.error { color: #ff6666; margin-bottom: 0.5rem; }
button { padding: 0.5rem 1rem; border-radius: 6px; }
</style>