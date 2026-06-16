import { describe, expect, it } from "vitest";

import { isDraftDirty } from "./draftDirtyState";

describe("template dirty guard", () => {
  it("preserves edits when draft differs from generated baseline", () => {
    expect(isDraftDirty("Edited draft", "Generated draft")).toBe(true);
    expect(isDraftDirty("Same text", "Same text")).toBe(false);
  });

  it("treats null baseline as not dirty", () => {
    expect(isDraftDirty("Manual only", null)).toBe(false);
  });
});
