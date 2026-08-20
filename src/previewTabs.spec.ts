import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { nextPreviewMode } from "./previewTabs.ts";

describe("nextPreviewMode", () => {
  it("moves right and wraps to the first tab", () => {
    assert.equal(nextPreviewMode("json", "ArrowRight"), "vtt");
    assert.equal(nextPreviewMode("chat", "ArrowRight"), "json");
  });

  it("moves left and wraps to the last tab", () => {
    assert.equal(nextPreviewMode("vtt", "ArrowLeft"), "json");
    assert.equal(nextPreviewMode("json", "ArrowLeft"), "chat");
  });

  it("supports Home and End", () => {
    assert.equal(nextPreviewMode("count", "Home"), "json");
    assert.equal(nextPreviewMode("count", "End"), "chat");
  });

  it("leaves unrelated keys to the browser", () => {
    assert.equal(nextPreviewMode("json", "Tab"), null);
  });
});
