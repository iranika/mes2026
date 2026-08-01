import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { createRetryableAsyncLoader } from "./retryableAsyncLoader.ts";

describe("createRetryableAsyncLoader", () => {
  it("retries after a failed load and caches the later success", async () => {
    let attempts = 0;
    const load = createRetryableAsyncLoader(async () => {
      attempts += 1;
      if (attempts === 1) throw new Error("temporary failure");
      return "ready";
    });

    await assert.rejects(load(), /temporary failure/);
    assert.equal(await load(), "ready");
    assert.equal(await load(), "ready");
    assert.equal(attempts, 2);
  });

  it("shares one in-flight attempt between concurrent callers", async () => {
    let resolveLoad: ((value: string) => void) | undefined;
    const load = createRetryableAsyncLoader(
      () => new Promise((resolve) => {
        resolveLoad = resolve;
      }),
    );

    const first = load();
    const second = load();
    assert.equal(first, second);

    resolveLoad?.("ready");
    assert.equal(await first, "ready");
  });
});
