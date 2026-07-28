export type LatestRequest = {
  isCurrent: () => boolean;
};

/**
 * Issues monotonically newer request handles so asynchronous work can ignore
 * results that were superseded while it was running.
 */
export function createLatestRequestTracker() {
  let latest = 0;

  return {
    next(): LatestRequest {
      const id = ++latest;
      return {
        isCurrent: () => id === latest,
      };
    },
  };
}
