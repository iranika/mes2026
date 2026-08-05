import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { describe, it } from "node:test";

const config = JSON.parse(
  readFileSync(new URL("./tauri.conf.json", import.meta.url), "utf8"),
);

describe("Tauri content security policy", () => {
  it("restricts production sources while allowing the bundled WASM app", () => {
    const csp = config.app.security.csp;

    assert.ok(csp, "production CSP should be enabled");
    assert.deepEqual(csp["connect-src"], ["ipc:", "http://ipc.localhost"]);
    assert.ok(csp["script-src"].includes("'wasm-unsafe-eval'"));
    assert.deepEqual(csp["object-src"], ["'none'"]);
    assert.deepEqual(csp["frame-ancestors"], ["'none'"]);
    assert.ok(config.app.security.devCsp["connect-src"].includes("ws:"));
    assert.ok(!csp["connect-src"].includes("ws:"));
  });
});
