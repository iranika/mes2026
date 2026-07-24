import { escapeHtml } from "./highlightMes.ts";

const SAFE_CHAT_SPAN =
  /&lt;span style=&quot;color:(#[0-9a-fA-F]{6})&quot;&gt;/g;

function decodeHtmlEntitiesOnce(html: string): string {
  const entities: Record<string, string> = {
    "&amp;": "&",
    "&lt;": "<",
    "&gt;": ">",
    "&quot;": '"',
    "&#39;": "'",
  };
  return html.replace(/&(amp|lt|gt|quot|#39);/g, (entity) => entities[entity]);
}

/**
 * Escape generated chat HTML, then restore only the fixed span markup emitted
 * by mes-core. This keeps older bundled WASM output safe as well.
 */
export function sanitizeChatHtml(html: string): string {
  return escapeHtml(decodeHtmlEntitiesOnce(html))
    .replace(SAFE_CHAT_SPAN, '<span style="color:$1">')
    .replace(/&lt;\/span&gt;/g, "</span>");
}
