import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { createChatExportDocument } from "./chatExport.ts";
import { sanitizeChatHtml } from "./sanitizeChatHtml.ts";

describe("sanitizeChatHtml", () => {
  it("keeps mes-core chat spans", () => {
    assert.equal(
      sanitizeChatHtml('<span style="color:#e11d48">Alice: hello</span>'),
      '<span style="color:#e11d48">Alice: hello</span>',
    );
  });

  it("escapes injected HTML while preserving allowed chat markup", () => {
    const html =
      '<span style="color:#e11d48"><img src=x onerror="alert(1)">: ' +
      '<script>alert(2)</script></span>';
    const sanitized = sanitizeChatHtml(html);

    assert.match(sanitized, /^<span style="color:#e11d48">/);
    assert.doesNotMatch(sanitized, /<img|<script/);
    assert.match(sanitized, /&lt;img src=x onerror=&quot;alert\(1\)&quot;&gt;/);
    assert.match(sanitized, /&lt;script&gt;alert\(2\)&lt;\/script&gt;/);
  });

  it("does not double-escape safe output from current mes-core", () => {
    const html =
      '<span style="color:#e11d48">&lt;script&gt; &amp; &quot;quoted&quot;</span>';

    assert.equal(
      sanitizeChatHtml(html),
      '<span style="color:#e11d48">&lt;script&gt; &amp; &quot;quoted&quot;</span>',
    );
  });

  it("does not allow arbitrary styles or attributes", () => {
    const html =
      '<span style="color:red" onclick="alert(1)">unsafe</span>' +
      '<span style="color:#2563eb">safe</span>';
    const sanitized = sanitizeChatHtml(html);

    assert.doesNotMatch(sanitized, /<span[^>]*onclick=/);
    assert.match(sanitized, /&lt;span style=&quot;color:red&quot;/);
    assert.match(sanitized, /<span style="color:#2563eb">safe<\/span>/);
  });
});

describe("createChatExportDocument", () => {
  it("preserves chat line breaks in a safe standalone document", () => {
    const contents =
      '<span style="color:#e11d48">Alice: hello</span>\n' +
      '<span style="color:#2563eb">Bob: <script>alert(1)</script></span>';
    const document = createChatExportDocument(contents);

    assert.match(document, /^<!doctype html>/);
    assert.match(document, /<meta charset="utf-8">/);
    assert.match(document, /white-space: pre-wrap/);
    assert.match(document, /Alice: hello<\/span>\n<span style="color:#2563eb">Bob:/);
    assert.doesNotMatch(document, /<script>/);
    assert.match(document, /&lt;script&gt;alert\(1\)&lt;\/script&gt;/);
  });
});
