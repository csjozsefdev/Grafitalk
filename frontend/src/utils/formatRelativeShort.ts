export function formatRelativeShort(timestampMs: number): string {
  const diffMs = Date.now() - timestampMs;
  if (diffMs < 60_000) {
    return "Just now";
  }
  if (diffMs < 3_600_000) {
    const minutes = Math.floor(diffMs / 60_000);
    return `${minutes} min ago`;
  }
  if (diffMs < 86_400_000) {
    const hours = Math.floor(diffMs / 3_600_000);
    return `${hours} hr ago`;
  }
  return new Date(timestampMs).toLocaleDateString();
}

export function formatRelativeFromIso(isoTimestamp: string | null): string | null {
  if (!isoTimestamp) {
    return null;
  }

  const timestampMs = Date.parse(isoTimestamp);
  if (Number.isNaN(timestampMs)) {
    return null;
  }

  return formatRelativeShort(timestampMs);
}
