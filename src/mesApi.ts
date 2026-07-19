export type PreviewMode = "json" | "vtt" | "count" | "chat";
export type MesBackend = "tauri" | "wasm";

type WasmModule = typeof import("./wasm/mes-core/mes_core.js");

const TAURI_COMMANDS: Record<PreviewMode, string> = {
  json: "mes_to_medo",
  vtt: "mes_to_vtt",
  count: "mes_word_count",
  chat: "mes_to_chat",
};

let wasmModule: WasmModule | null = null;
let wasmInit: Promise<WasmModule> | null = null;

function isTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export function getMesBackend(): MesBackend {
  return isTauriRuntime() ? "tauri" : "wasm";
}

async function loadWasm(): Promise<WasmModule> {
  if (wasmModule) return wasmModule;
  if (!wasmInit) {
    wasmInit = (async () => {
      const mod = await import("./wasm/mes-core/mes_core.js");
      await mod.default();
      wasmModule = mod;
      return mod;
    })();
  }
  return wasmInit;
}

async function convertWithWasm(
  mod: WasmModule,
  mode: PreviewMode,
  text: string,
): Promise<string> {
  switch (mode) {
    case "json":
      return await mod.parse_mes_to_json(text);
    case "vtt":
      return await mod.get_vtt(text);
    case "count":
      return await mod.count_dialogue_word_to_json(text);
    case "chat":
      return await mod.get_chat(text);
  }
}

/** Convert MeS text via Tauri when available, otherwise via in-browser WASM. */
export async function convertMes(mode: PreviewMode, text: string): Promise<string> {
  if (isTauriRuntime()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return (await invoke(TAURI_COMMANDS[mode], { text })) as string;
  }

  const mod = await loadWasm();
  return await convertWithWasm(mod, mode, text);
}

/** Parse then emit a canonical MeS script (flat dialogue expanded, decorators normalized). */
export async function emitMes(text: string): Promise<string> {
  if (isTauriRuntime()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return (await invoke("mes_emit", { text })) as string;
  }

  const mod = await loadWasm();
  return await mod.emit_mes(text);
}
