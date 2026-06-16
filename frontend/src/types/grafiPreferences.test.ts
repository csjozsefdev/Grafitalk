import { afterEach, describe, expect, it } from "vitest";

import {
  DEFAULT_GRAFI_TALK_PREFERENCES,
  loadGrafiTalkPreferences,
  saveGrafiTalkPreferences,
} from "./grafiPreferences";

const STORAGE_KEY = "grafitalk.preferences.v1";

describe("grafiPreferences", () => {
  afterEach(() => {
    window.localStorage.clear();
  });

  it("migrates legacy soundEnabled to readAloudEnabled", () => {
    window.localStorage.setItem(
      STORAGE_KEY,
      JSON.stringify({
        enabled: true,
        motionEnabled: true,
        soundEnabled: false,
        silentMode: false,
        criticalAlertsOnly: false,
      })
    );

    expect(loadGrafiTalkPreferences().readAloudEnabled).toBe(false);
  });

  it("persists GrafiTalk preferences", () => {
    saveGrafiTalkPreferences({
      ...DEFAULT_GRAFI_TALK_PREFERENCES,
      enabled: false,
      readAloudEnabled: false,
      voiceSelection: "system-default",
    });

    expect(loadGrafiTalkPreferences()).toEqual({
      ...DEFAULT_GRAFI_TALK_PREFERENCES,
      enabled: false,
      readAloudEnabled: false,
    });
  });

  it("defaults voiceSelection when missing from storage", () => {
    window.localStorage.setItem(
      STORAGE_KEY,
      JSON.stringify({
        enabled: true,
        motionEnabled: true,
        readAloudEnabled: true,
        silentMode: false,
        criticalAlertsOnly: false,
      })
    );

    expect(loadGrafiTalkPreferences().voiceSelection).toBe("system-default");
  });
});
