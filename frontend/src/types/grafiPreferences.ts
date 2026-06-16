import type { GrafiSettings } from "../vendor/grafi/grafiTypes";

export type GrafiTalkVoiceSelection = "system-default";

export interface GrafiTalkPreferences {
  enabled: boolean;
  motionEnabled: boolean;
  readAloudEnabled: boolean;
  silentMode: boolean;
  criticalAlertsOnly: boolean;
  voiceSelection: GrafiTalkVoiceSelection;
}

const STORAGE_KEY = "grafitalk.preferences.v1";

export const DEFAULT_GRAFI_TALK_PREFERENCES: GrafiTalkPreferences = {
  enabled: true,
  motionEnabled: true,
  readAloudEnabled: true,
  silentMode: false,
  criticalAlertsOnly: false,
  voiceSelection: "system-default",
};

export function toGrafiSettings(preferences: GrafiTalkPreferences): GrafiSettings {
  return {
    enabled: preferences.enabled,
    motionEnabled: preferences.motionEnabled,
    soundEnabled: false,
    silentMode: preferences.silentMode,
    criticalAlertsOnly: preferences.criticalAlertsOnly,
  };
}

export function loadGrafiTalkPreferences(): GrafiTalkPreferences {
  if (typeof window === "undefined") {
    return DEFAULT_GRAFI_TALK_PREFERENCES;
  }

  try {
    const raw = window.localStorage.getItem(STORAGE_KEY);
    if (!raw) {
      return DEFAULT_GRAFI_TALK_PREFERENCES;
    }

    const parsed = JSON.parse(raw) as Record<string, unknown>;
    return {
      enabled:
        typeof parsed.enabled === "boolean"
          ? parsed.enabled
          : DEFAULT_GRAFI_TALK_PREFERENCES.enabled,
      motionEnabled:
        typeof parsed.motionEnabled === "boolean"
          ? parsed.motionEnabled
          : DEFAULT_GRAFI_TALK_PREFERENCES.motionEnabled,
      readAloudEnabled:
        typeof parsed.readAloudEnabled === "boolean"
          ? parsed.readAloudEnabled
          : typeof parsed.soundEnabled === "boolean"
            ? parsed.soundEnabled
            : DEFAULT_GRAFI_TALK_PREFERENCES.readAloudEnabled,
      silentMode:
        typeof parsed.silentMode === "boolean"
          ? parsed.silentMode
          : DEFAULT_GRAFI_TALK_PREFERENCES.silentMode,
      criticalAlertsOnly:
        typeof parsed.criticalAlertsOnly === "boolean"
          ? parsed.criticalAlertsOnly
          : DEFAULT_GRAFI_TALK_PREFERENCES.criticalAlertsOnly,
      voiceSelection:
        parsed.voiceSelection === "system-default"
          ? "system-default"
          : DEFAULT_GRAFI_TALK_PREFERENCES.voiceSelection,
    };
  } catch {
    return DEFAULT_GRAFI_TALK_PREFERENCES;
  }
}

export function saveGrafiTalkPreferences(preferences: GrafiTalkPreferences): void {
  window.localStorage.setItem(STORAGE_KEY, JSON.stringify(preferences));
}
