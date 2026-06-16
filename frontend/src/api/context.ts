import { invoke } from "@tauri-apps/api/core";

export async function getProjectContext(projectId: string): Promise<string> {
  return invoke<string>("get_project_context", { projectId });
}

export async function saveProjectContext(
  projectId: string,
  contextText: string
): Promise<void> {
  return invoke<void>("save_project_context", {
    projectId,
    contextText,
  });
}
