import { renderHook } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { useGrafiFlash } from "./useGrafiFlash";
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

describe("useGrafiFlash", () => {
  it("emits transient flash messages", () => {
    const { result, rerender } = renderHook(
      (input) => useGrafiFlash(input),
      { initialProps: baseInput }
    );

    result.current.emitGrafiEvent("copySucceeded");
    rerender(baseInput);

    expect(result.current.message?.text).toBe("Copied to clipboard.");
    expect(result.current.message?.transient).toBe(true);
  });
});
