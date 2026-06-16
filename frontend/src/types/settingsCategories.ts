export type SettingsCategoryId =
  | "general"
  | "read-aloud"
  | "grafi"
  | "export"
  | "about";

export interface SettingsCategory {
  id: SettingsCategoryId;
  label: string;
}

export const SETTINGS_CATEGORIES: SettingsCategory[] = [
  { id: "general", label: "General" },
  { id: "read-aloud", label: "Read Aloud" },
  { id: "grafi", label: "Grafi" },
  { id: "export", label: "Export" },
  { id: "about", label: "About / Diagnostics" },
];
