import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { canDiscardChanges, hasUnsavedChanges } from "./unsavedChanges.ts";

describe("hasUnsavedChanges", () => {
  it("reports an exact match as saved", () => {
    assert.equal(hasUnsavedChanges("@Alice\nhello\n", "@Alice\nhello\n"), false);
  });

  it("detects edits that would be discarded", () => {
    assert.equal(hasUnsavedChanges("@Alice\nhello!\n", "@Alice\nhello\n"), true);
  });

  it("does not prompt when there are no edits", () => {
    let prompted = false;
    const allowed = canDiscardChanges("same", "same", () => {
      prompted = true;
      return false;
    });

    assert.equal(allowed, true);
    assert.equal(prompted, false);
  });

  it("keeps unsaved edits when confirmation is declined", () => {
    assert.equal(canDiscardChanges("edited", "saved", () => false), false);
  });

  it("allows unsaved edits to be discarded after confirmation", () => {
    assert.equal(canDiscardChanges("edited", "saved", () => true), true);
  });
});
