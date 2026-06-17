import { describe, expect, it } from "vitest";

import { shouldAutoExpandGrafiMessage } from "./grafiTalkAdvisorUtils";

describe("shouldAutoExpandGrafiMessage", () => {
  it("expands transient and medium-or-higher priority messages", () => {
    expect(
      shouldAutoExpandGrafiMessage({
        id: "welcome",
        text: "Hi",
        priority: "low",
      })
    ).toBe(false);

    expect(
      shouldAutoExpandGrafiMessage({
        id: "empty-context",
        text: "Add context",
        priority: "medium",
      })
    ).toBe(true);

    expect(
      shouldAutoExpandGrafiMessage({
        id: "copy-success::1",
        text: "Copied",
        priority: "low",
        transient: true,
      })
    ).toBe(true);
  });
});
