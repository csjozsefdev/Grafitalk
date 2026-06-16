import { invoke } from "@tauri-apps/api/core";

export type ImportFormatHint = "json" | "text" | "markdown";

function formatHintFromPath(path: string): ImportFormatHint | undefined {
  const lower = path.toLowerCase();
  if (lower.endsWith(".json")) return "json";
  if (lower.endsWith(".md") || lower.endsWith(".markdown")) return "markdown";
  if (lower.endsWith(".txt")) return "text";
  return undefined;
}

export async function importGrafIdHandoff(
  projectId: string,
  content: string,
  options?: {
    formatHint?: ImportFormatHint;
    pathHint?: string;
  }
): Promise<string> {
  return invoke<string>("import_graf_id_handoff", {
    projectId,
    content,
    formatHint: options?.formatHint ?? null,
    pathHint: options?.pathHint ?? null,
  });
}

export { formatHintFromPath };
