export function normalizeDraftText(text: string): string {
  return text.replace(/\r\n/g, "\n").trimEnd();
}

export function isDraftDirty(
  currentDraft: string,
  generatedBaseline: string | null
): boolean {
  if (generatedBaseline === null) {
    return false;
  }

  return (
    normalizeDraftText(currentDraft) !== normalizeDraftText(generatedBaseline)
  );
}
