import { describe, expect, it } from "vitest";

import { deriveWorkflowStep } from "./deriveWorkflowStep";
import { isDraftDirty, normalizeDraftText } from "./draftDirtyState";

describe("deriveWorkflowStep", () => {
  it("returns context when no project is selected", () => {
    expect(
      deriveWorkflowStep({
        selectedId: null,
        hasContext: false,
        draftReady: false,
        isGenerating: false,
        sessionCompletion: "none",
      })
    ).toBe("context");
  });

  it("returns template when context exists but draft does not", () => {
    expect(
      deriveWorkflowStep({
        selectedId: "p1",
        hasContext: true,
        draftReady: false,
        isGenerating: false,
        sessionCompletion: "none",
      })
    ).toBe("template");
  });

  it("returns review when draft is ready", () => {
    expect(
      deriveWorkflowStep({
        selectedId: "p1",
        hasContext: true,
        draftReady: true,
        isGenerating: false,
        sessionCompletion: "none",
      })
    ).toBe("review");
  });

  it("returns copy after export completion", () => {
    expect(
      deriveWorkflowStep({
        selectedId: "p1",
        hasContext: true,
        draftReady: true,
        isGenerating: false,
        sessionCompletion: "exported",
      })
    ).toBe("copy");
  });
});

describe("draftDirtyState", () => {
  it("detects manual edits against generated baseline", () => {
    expect(isDraftDirty("Hello client", "Hello")).toBe(true);
    expect(isDraftDirty("Hello", "Hello")).toBe(false);
  });

  it("normalizes line endings", () => {
    expect(normalizeDraftText("Hello\r\n")).toBe("Hello");
  });
});
