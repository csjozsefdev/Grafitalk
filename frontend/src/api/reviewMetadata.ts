import { invoke } from "@tauri-apps/api/core";

import type { TemplateKind } from "../types/template";
import { isTemplateKind, templateTitle } from "../types/template";

export interface ProjectReviewMetadata {
  lastGeneratedAt: string | null;
  lastTemplateKind: TemplateKind;
  lastExportedAt: string | null;
  lastExportFormat: string | null;
}

export async function getProjectReviewMetadata(
  projectId: string
): Promise<ProjectReviewMetadata> {
  const payload = await invoke<ProjectReviewMetadata>(
    "get_project_review_metadata",
    { projectId }
  );

  return {
    ...payload,
    lastTemplateKind: isTemplateKind(payload.lastTemplateKind)
      ? payload.lastTemplateKind
      : "status_update",
  };
}

export async function migrateLastGeneratedAtIfEmpty(
  projectId: string,
  legacyTimestamp: string
): Promise<boolean> {
  return invoke<boolean>("migrate_last_generated_at_if_empty", {
    projectId,
    legacyTimestamp,
  });
}

export async function recordDraftExport(
  projectId: string,
  exportFormat: string
): Promise<void> {
  await invoke<void>("record_draft_export", {
    projectId,
    exportFormat,
  });
}

export function formatReviewTemplateLabel(kind: TemplateKind): string {
  return templateTitle(kind);
}

export function exportFormatFromPath(path: string): string {
  const lower = path.toLowerCase();
  if (lower.endsWith(".pdf")) return "pdf";
  if (lower.endsWith(".json")) return "json";
  if (lower.endsWith(".md") || lower.endsWith(".markdown")) return "md";
  if (lower.endsWith(".txt")) return "txt";
  return "txt";
}
