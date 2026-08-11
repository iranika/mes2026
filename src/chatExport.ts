import { sanitizeChatHtml } from "./sanitizeChatHtml.ts";

/** Build a standalone HTML document while keeping one generated chat entry per line. */
export function createChatExportDocument(contents: string): string {
  const chat = sanitizeChatHtml(contents);
  return `<!doctype html>
<html lang="ja">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>MeS Chat</title>
  <style>
    body { margin: 1rem; font-family: system-ui, sans-serif; }
    .chat { white-space: pre-wrap; line-height: 1.7; overflow-wrap: anywhere; }
  </style>
</head>
<body>
  <main class="chat">${chat}</main>
</body>
</html>
`;
}
