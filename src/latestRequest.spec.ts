import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { createLatestRequestTracker } from "./latestRequest.ts";

describe("createLatestRequestTracker", () => {
  it("marks an older asynchronous request stale as soon as a newer one starts", async () => {
    const tracker = createLatestRequestTracker();
    const first = tracker.next();

    await Promise.resolve();
    const second = tracker.next();

    assert.equal(first.isCurrent(), false);
    assert.equal(second.isCurrent(), true);
  });

  it("keeps the active request current across asynchronous work", async () => {
    const tracker = createLatestRequestTracker();
    const request = tracker.next();

    await Promise.resolve();

    assert.equal(request.isCurrent(), true);
  });
});
