const STORAGE_KEY = "grafitalk:lastGeneratedByProject";

export function loadLastGeneratedByProject(): Record<string, number> {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) {
      return {};
    }

    const parsed: unknown = JSON.parse(raw);
    if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
      return {};
    }

    const result: Record<string, number> = {};
    for (const [projectId, timestamp] of Object.entries(parsed)) {
      if (typeof timestamp === "number" && Number.isFinite(timestamp)) {
        result[projectId] = timestamp;
      }
    }
    return result;
  } catch (err) {
    console.error("Failed to load last generated timestamps", err);
    return {};
  }
}
