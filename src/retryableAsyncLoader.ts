/** Cache a successful async load while allowing retries after a rejection. */
export function createRetryableAsyncLoader<T>(load: () => Promise<T>): () => Promise<T> {
  let loaded: T | undefined;
  let hasLoaded = false;
  let pending: Promise<T> | null = null;

  return () => {
    if (hasLoaded) return Promise.resolve(loaded as T);
    if (!pending) {
      pending = load().then(
        (value) => {
          loaded = value;
          hasLoaded = true;
          return value;
        },
        (error) => {
          pending = null;
          throw error;
        },
      );
    }
    return pending;
  };
}
