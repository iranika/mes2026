/** Return whether the editor differs from the last loaded or saved source. */
export function hasUnsavedChanges(current: string, saved: string): boolean {
  return current !== saved;
}

/** Allow replacement immediately when clean, otherwise defer to confirmation. */
export function canDiscardChanges(
  current: string,
  saved: string,
  confirmDiscard: () => boolean,
): boolean {
  return !hasUnsavedChanges(current, saved) || confirmDiscard();
}
