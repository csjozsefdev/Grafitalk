import { invoke } from "@tauri-apps/api/core";

export interface AppDiagnostics {
  appVersion: string;
  dbPath: string;
  projectCount: number;
  latestMigration: number | null;
  platform: string;
}

export async function getAppDiagnostics(): Promise<AppDiagnostics> {
  return invoke<AppDiagnostics>("get_app_diagnostics");
}

export async function exportDatabaseBackup(destinationPath: string): Promise<void> {
  await invoke<void>("export_database_backup", { destinationPath });
}

export async function openDataFolder(): Promise<string> {
  return invoke<string>("open_data_folder");
}

export function formatDiagnosticsText(diagnostics: AppDiagnostics): string {
  return [
    `GrafiTalk ${diagnostics.appVersion}`,
    `Platform: ${diagnostics.platform}`,
    `Database: ${diagnostics.dbPath}`,
    `Active projects: ${diagnostics.projectCount}`,
    `Latest migration: ${diagnostics.latestMigration ?? "unknown"}`,
  ].join("\n");
}
