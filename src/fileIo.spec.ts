import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { deferRevokeObjectUrl } from "./objectUrl.ts";

describe("browser downloads", () => {
  it("keeps the Blob URL alive until the click default action can start", async () => {
    const originalRevokeObjectURL = URL.revokeObjectURL;
    const url = "blob:mes-download";
    let revokedUrl: string | null = null;

    URL.revokeObjectURL = (value) => {
      revokedUrl = value;
    };

    try {
      deferRevokeObjectUrl(url);
      assert.equal(revokedUrl, null);

      await new Promise((resolve) => setTimeout(resolve, 0));
      assert.equal(revokedUrl, url);
    } finally {
      URL.revokeObjectURL = originalRevokeObjectURL;
    }
  });
});
