const SAFE_CHAT_TAG =
  /<span style="color:(#[0-9a-fA-F]{6})">|<\/span>/g;

function escapeHtmlPreservingEntities(text: string): string {
  return text
    .replace(/&(?!(?:amp|lt|gt|quot|#39);)/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

/**
 * Preserve only the raw, fixed span tags emitted by mes-core. Already escaped
 * entities belong to the dialogue text and must not be promoted back to tags.
 */
export function sanitizeChatHtml(html: string): string {
  let sanitized = "";
  let cursor = 0;

  for (const match of html.matchAll(SAFE_CHAT_TAG)) {
    const index = match.index ?? 0;
    sanitized += escapeHtmlPreservingEntities(html.slice(cursor, index));
    sanitized += match[1]
      ? `<span style="color:${match[1]}">`
      : "</span>";
    cursor = index + match[0].length;
  }

  return sanitized + escapeHtmlPreservingEntities(html.slice(cursor));
}
