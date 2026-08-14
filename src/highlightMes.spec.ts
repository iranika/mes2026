import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { escapeHtml, highlightMes } from "./highlightMes.ts";

describe("highlightMes", () => {
  it("escapes HTML specials", () => {
    assert.equal(escapeHtml("<a&b>"), "&lt;a&amp;b&gt;");
  });

  it("highlights MeS prefixes and delimiter", () => {
    const html = highlightMes(
      "@Alice\n# note\n$ sfx\n! pos\n&00:00:01.000 --> 00:00:02.000\n----\nhello",
    );
    assert.match(html, /class="tok-char"/);
    assert.match(html, /class="tok-comment"/);
    assert.match(html, /class="tok-sound"/);
    assert.match(html, /class="tok-pos"/);
    assert.match(html, /class="tok-timing"/);
    assert.match(html, /class="tok-delimiter"/);
    assert.doesNotMatch(html, /<script/);
  });

  it("normalizes CRLF before highlighting Windows files", () => {
    assert.equal(
      highlightMes("@Alice\r\n----\r\nhello"),
      '<span class="tok-char"><span class="tok-prefix">@</span>Alice</span>\n' +
        '<span class="tok-delimiter">----</span>\nhello',
    );
  });
});

describe("App accessibility", () => {
  it("announces status updates and errors to assistive technology", () => {
    const source = readFileSync(new URL("./App.vue", import.meta.url), "utf8");

    assert.match(source, /<p[^>]*v-if="status"[^>]*role="status"[^>]*>/);
    assert.match(source, /<div[^>]*v-if="error"[^>]*role="alert"[^>]*>/);
  });
});
