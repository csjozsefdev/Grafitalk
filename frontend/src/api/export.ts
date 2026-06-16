import { invoke } from "@tauri-apps/api/core";

import type { TemplateKind } from "../types/template";

export interface PreparedExport {
  suggestedFilename: string;
  text: string | null;
  bytes: number[] | null;
}

export async function prepareDraftExport(
  projectId: string,
  templateKind: TemplateKind,
  draftText: string,
  options?: {
    formatHint?: string;
    pathHint?: string;
  }
): Promise<PreparedExport> {
  return invoke<PreparedExport>("prepare_draft_export", {
    projectId,
    templateKind,
    draftText,
    formatHint: options?.formatHint ?? null,
    pathHint: options?.pathHint ?? null,
  });
}

export function buildSuggestedExportFilename(
  projectName: string,
  templateKind: TemplateKind,
  extension: "txt" | "md" | "json" | "pdf" = "txt"
): string {
  const projectSlug = slugify(projectName);
  return `grafitalk_${projectSlug}_${templateKind}.${extension}`;
}

function slugify(value: string): string {
  const slug = value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "_")
    .replace(/^_+|_+$/g, "");
  return slug || "draft";
}
