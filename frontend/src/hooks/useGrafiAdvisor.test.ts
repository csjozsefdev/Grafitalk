import { describe, expect, it } from "vitest";

import {
  grafiMessagePriorityRank,
  resolveGrafiAdvisorMessage,
} from "./useGrafiAdvisor";
import { DEFAULT_GRAFI_TALK_PREFERENCES } from "../types/grafiPreferences";

const baseInput = {
  preferences: DEFAULT_GRAFI_TALK_PREFERENCES,
  selectedId: null as string | null,
  hasContext: false,
  draftReady: false,
  isDraftDirty: false,
  sessionCompletion: "none" as const,
  activeProjectCount: 0,
  allProjectsArchived: false,
};

describe("resolveGrafiAdvisorMessage", () => {
  it("shows welcome guidance when no project is selected", () => {
    const message = resolveGrafiAdvisorMessage(baseInput);
    expect(message?.id).toBe("welcome");
  });

  it("shows draft-ready guidance when a draft exists", () => {
    const message = resolveGrafiAdvisorMessage({
      ...baseInput,
      selectedId: "p1",
      hasContext: true,
      draftReady: true,
    });
    expect(message?.id).toBe("draft-ready");
  });

  it("hides non-critical guidance in critical-only mode", () => {
    const message = resolveGrafiAdvisorMessage({
      ...baseInput,
      preferences: {
        ...DEFAULT_GRAFI_TALK_PREFERENCES,
        criticalAlertsOnly: true,
      },
    });
    expect(message).toBeNull();
  });
});

describe("useGrafiAdvisor helpers", () => {
  it("ranks high priority above low", () => {
    expect(grafiMessagePriorityRank("high")).toBeGreaterThan(
      grafiMessagePriorityRank("low")
    );
  });
});
