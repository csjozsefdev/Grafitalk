export function formatLastUsed(iso: string | null): string {
  if (!iso) {
    return "Never used";
  }

  const date = new Date(iso);
  if (Number.isNaN(date.getTime())) {
    return "Unknown";
  }

  return date.toLocaleString();
}
