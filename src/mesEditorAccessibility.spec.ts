import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { describe, it } from "node:test";

const source = readFileSync(
  new URL("./components/MesEditor.vue", import.meta.url),
  "utf8",
);

describe("MeS editor forced-colors fallback", () => {
  it("shows the textarea text and hides the highlighting overlay", () => {
    const start = source.indexOf("@media (forced-colors: active)");
    assert.notEqual(start, -1, "forced-colors styles should be defined");

    const forcedColorsStyles = source.slice(start);
    assert.match(
      forcedColorsStyles,
      /\.backdrop\s*{[^}]*display:\s*none;/s,
    );
    assert.match(
      forcedColorsStyles,
      /\.input\s*{[^}]*color:\s*CanvasText;/s,
    );
    assert.match(
      forcedColorsStyles,
      /\.input::selection\s*{[^}]*color:\s*HighlightText;[^}]*background:\s*Highlight;/s,
    );
  });
});
