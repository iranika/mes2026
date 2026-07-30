/** Release a download URL after the browser has processed the anchor click. */
export function deferRevokeObjectUrl(url: string): void {
  setTimeout(() => URL.revokeObjectURL(url), 0);
}
