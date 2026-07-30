import { getMesBackend } from "./mesApi";
import { deferRevokeObjectUrl } from "./objectUrl";

export type ExportKind = "mes" | "json" | "vtt" | "count" | "chat";

function isTauri(): boolean {
  return getMesBackend() === "tauri";
}

function downloadBrowser(filename: string, contents: string, mime = "text/plain;charset=utf-8") {
  const blob = new Blob([contents], { type: mime });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  deferRevokeObjectUrl(url);
}

function pickBrowserFile(accept: string): Promise<File | null> {
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = accept;
    input.onchange = () => {
      resolve(input.files?.[0] ?? null);
    };
    input.oncancel = () => resolve(null);
    input.click();
  });
}

export type OpenResult = { path: string | null; contents: string };

/** Open a MeS (or text) file via native dialog or browser file picker. */
export async function openMesFile(): Promise<OpenResult | null> {
  if (isTauri()) {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const { readTextFile } = await import("@tauri-apps/plugin-fs");
    const selected = await open({
      multiple: false,
      filters: [
        { name: "MeS", extensions: ["mes", "txt"] },
        { name: "All", extensions: ["*"] },
      ],
    });
    if (!selected || Array.isArray(selected)) return null;
    const contents = await readTextFile(selected);
    return { path: selected, contents };
  }

  const file = await pickBrowserFile(".mes,.txt,text/plain");
  if (!file) return null;
  return { path: file.name, contents: await file.text() };
}

/** Save MeS source. In Tauri, uses a save dialog (or overwrite known path). */
export async function saveMesFile(
  contents: string,
  currentPath: string | null,
): Promise<string | null> {
  if (isTauri()) {
    const { save } = await import("@tauri-apps/plugin-dialog");
    const { writeTextFile } = await import("@tauri-apps/plugin-fs");
    let path = currentPath;
    if (!path) {
      path =
        (await save({
          filters: [{ name: "MeS", extensions: ["mes"] }],
          defaultPath: "script.mes",
        })) ?? null;
    }
    if (!path) return null;
    await writeTextFile(path, contents);
    return path;
  }

  downloadBrowser(currentPath || "script.mes", contents);
  return currentPath || "script.mes";
}

/** Save As — always prompt for a destination in Tauri; download in browser. */
export async function saveMesFileAs(contents: string): Promise<string | null> {
  if (isTauri()) {
    const { save } = await import("@tauri-apps/plugin-dialog");
    const { writeTextFile } = await import("@tauri-apps/plugin-fs");
    const path = await save({
      filters: [{ name: "MeS", extensions: ["mes"] }],
      defaultPath: "script.mes",
    });
    if (!path) return null;
    await writeTextFile(path, contents);
    return path;
  }

  downloadBrowser("script.mes", contents);
  return "script.mes";
}

const EXPORT_META: Record<
  Exclude<ExportKind, "mes">,
  { filename: string; mime: string; filterName: string; extensions: string[] }
> = {
  json: {
    filename: "preview.json",
    mime: "application/json;charset=utf-8",
    filterName: "JSON",
    extensions: ["json"],
  },
  vtt: {
    filename: "preview.vtt",
    mime: "text/vtt;charset=utf-8",
    filterName: "VTT",
    extensions: ["vtt"],
  },
  count: {
    filename: "word-count.json",
    mime: "application/json;charset=utf-8",
    filterName: "JSON",
    extensions: ["json"],
  },
  chat: {
    filename: "chat.html",
    mime: "text/html;charset=utf-8",
    filterName: "HTML",
    extensions: ["html"],
  },
};

/** Export the current preview payload. */
export async function exportPreview(
  kind: Exclude<ExportKind, "mes">,
  contents: string,
): Promise<boolean> {
  const meta = EXPORT_META[kind];
  if (isTauri()) {
    const { save } = await import("@tauri-apps/plugin-dialog");
    const { writeTextFile } = await import("@tauri-apps/plugin-fs");
    const path = await save({
      filters: [{ name: meta.filterName, extensions: meta.extensions }],
      defaultPath: meta.filename,
    });
    if (!path) return false;
    await writeTextFile(path, contents);
    return true;
  }

  downloadBrowser(meta.filename, contents, meta.mime);
  return true;
}
