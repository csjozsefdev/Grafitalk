import { invoke } from "@tauri-apps/api/core";

import type { TemplateKind } from "../types/template";
import { DEFAULT_TEMPLATE_KIND } from "../types/template";

export async function getProjectDraft(projectId: string): Promise<string> {
  return invoke<string>("get_project_draft", { projectId });
}

export async function saveProjectDraft(
  projectId: string,
  draftText: string
): Promise<void> {
  await invoke<void>("save_project_draft", { projectId, draftText });
}

export async function generateStatusDraft(projectId: string): Promise<string> {
  return invoke<string>("generate_status_draft", { projectId });
}

export async function generateDraft(
  projectId: string,
  templateKind: TemplateKind
): Promise<string> {
  return invoke<string>("generate_draft", { projectId, templateKind });
}

export async function getProjectTemplateKind(
  projectId: string
): Promise<TemplateKind> {
  const kind = await invoke<string>("get_project_template_kind", { projectId });
  return (kind as TemplateKind) || DEFAULT_TEMPLATE_KIND;
}

export async function saveProjectTemplateKind(
  projectId: string,
  templateKind: TemplateKind
): Promise<void> {
  await invoke<void>("save_project_template_kind", {
    projectId,
    templateKind,
  });
}
