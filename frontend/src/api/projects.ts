import { invoke } from "@tauri-apps/api/core";

import type { Project } from "../types/project";

export async function listProjects(): Promise<Project[]> {
  return invoke<Project[]>("list_projects");
}

export async function listArchivedProjects(): Promise<Project[]> {
  return invoke<Project[]>("list_archived_projects");
}

export async function createProject(
  name: string,
  clientLabel?: string
): Promise<Project> {
  return invoke<Project>("create_project", {
    name,
    clientLabel: clientLabel ?? null,
  });
}

export async function touchProjectUsed(projectId: string): Promise<void> {
  return invoke<void>("touch_project_used", { projectId });
}

export async function archiveProject(projectId: string): Promise<void> {
  return invoke<void>("archive_project", { projectId });
}

export async function restoreProject(projectId: string): Promise<void> {
  return invoke<void>("restore_project", { projectId });
}

export async function renameProject(
  projectId: string,
  name: string
): Promise<Project> {
  return invoke<Project>("rename_project", { projectId, name });
}

export const MAX_PROJECT_NAME_LEN = 120;
