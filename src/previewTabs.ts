import type { PreviewMode } from "./mesApi";

const PREVIEW_MODES: readonly PreviewMode[] = ["json", "vtt", "count", "chat"];

/** Resolve the automatically activated tab for WAI-ARIA keyboard navigation. */
export function nextPreviewMode(current: PreviewMode, key: string): PreviewMode | null {
  const index = PREVIEW_MODES.indexOf(current);
  switch (key) {
    case "ArrowRight":
      return PREVIEW_MODES[(index + 1) % PREVIEW_MODES.length];
    case "ArrowLeft":
      return PREVIEW_MODES[(index - 1 + PREVIEW_MODES.length) % PREVIEW_MODES.length];
    case "Home":
      return PREVIEW_MODES[0];
    case "End":
      return PREVIEW_MODES[PREVIEW_MODES.length - 1];
    default:
      return null;
  }
}
