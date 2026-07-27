/** Escape HTML special characters for safe overlay rendering. */
export function escapeHtml(text: string): string {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

const PREFIX_CLASS: Record<string, string> = {
  "@": "tok-char",
  "＠": "tok-char",
  "#": "tok-comment",
  "＃": "tok-comment",
  $: "tok-sound",
  "＄": "tok-sound",
  "!": "tok-pos",
  "！": "tok-pos",
  "&": "tok-timing",
  "＆": "tok-timing",
};

/**
 * Highlight MeS attribute prefixes and the header delimiter for the editor overlay.
 */
export function highlightMes(source: string): string {
  if (!source) return "\n";

  // Textarea values use LF internally. Normalize opened Windows files as well
  // so a trailing CR does not break delimiter detection or overlay alignment.
  return source
    .replace(/\r\n/g, "\n")
    .split("\n")
    .map((line) => {
      if (line === "----") {
        return `<span class="tok-delimiter">${escapeHtml(line)}</span>`;
      }

      const first = line.charAt(0);
      const cls = PREFIX_CLASS[first];
      if (!cls || !line) {
        return escapeHtml(line);
      }

      const prefix = escapeHtml(first);
      const rest = escapeHtml(line.slice(1));
      return `<span class="${cls}"><span class="tok-prefix">${prefix}</span>${rest}</span>`;
    })
    .join("\n");
}
