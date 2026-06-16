import { cleanup, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";

vi.mock("../vendor/grafi/assets/grafi-transparent.png", () => ({
  default: "grafi-transparent-mock.png",
}));

import {
  GrafiTalkAdvisor,
  shouldAutoExpandGrafiMessage,
} from "./GrafiTalkAdvisor";
import { DEFAULT_GRAFI_TALK_PREFERENCES } from "../types/grafiPreferences";

describe("GrafiTalkAdvisor", () => {
  afterEach(() => {
    cleanup();
    vi.useRealTimers();
    document.body.innerHTML = "";
  });

  it("portals upstream GrafiAdvisor into gt-grafi-host", () => {
    render(
      <GrafiTalkAdvisor
        message={{
          id: "draft-ready",
          text: "Your draft is ready.",
          priority: "low",
        }}
        preferences={DEFAULT_GRAFI_TALK_PREFERENCES}
      />
    );

    const host = document.body.querySelector(".gt-grafi-host");
    expect(host).not.toBeNull();
    expect(host?.querySelector(".grafi-advisor--placement-static")).not.toBeNull();
    expect(host?.querySelector(".grafi-bubble__figure-btn")).not.toBeNull();
    expect(screen.queryByText("Your draft is ready.")).not.toBeInTheDocument();
  });

  it("auto-expands high-priority and transient messages", () => {
    render(
      <GrafiTalkAdvisor
        message={{
          id: "export-failed::1",
          text: "Export failed.",
          priority: "high",
          transient: true,
        }}
        preferences={DEFAULT_GRAFI_TALK_PREFERENCES}
      />
    );

    expect(screen.getByText("Export failed.")).toBeInTheDocument();
    expect(
      document.body.querySelector(".grafi-advisor--expanded")
    ).not.toBeNull();
  });

  it("renders nothing when Grafi is disabled", () => {
    render(
      <GrafiTalkAdvisor
        message={{
          id: "welcome",
          text: "Create or select a project.",
          priority: "low",
        }}
        preferences={{ ...DEFAULT_GRAFI_TALK_PREFERENCES, enabled: false }}
      />
    );

    expect(document.body.querySelector(".gt-grafi-host")).toBeNull();
  });

  it("dismiss closes only the bubble, not Grafi", async () => {
    const user = userEvent.setup();

    render(
      <GrafiTalkAdvisor
        message={{
          id: "empty-context",
          text: "Add context notes.",
          priority: "medium",
        }}
        preferences={DEFAULT_GRAFI_TALK_PREFERENCES}
      />
    );

    expect(screen.getByText("Add context notes.")).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Dismiss Grafi message" }));

    expect(screen.queryByText("Add context notes.")).not.toBeInTheDocument();
    expect(document.body.querySelector(".gt-grafi-host")).not.toBeNull();
    expect(
      document.body.querySelector(".grafi-bubble__figure-btn")
    ).not.toBeNull();
    expect(
      document.body.querySelector(".grafi-advisor--minimized")
    ).not.toBeNull();
  });

  it("can reopen the bubble by clicking the figure", async () => {
    const user = userEvent.setup();

    render(
      <GrafiTalkAdvisor
        message={{
          id: "empty-context",
          text: "Add context notes.",
          priority: "medium",
        }}
        preferences={DEFAULT_GRAFI_TALK_PREFERENCES}
      />
    );

    await user.click(screen.getByRole("button", { name: "Dismiss Grafi message" }));
    expect(screen.queryByText("Add context notes.")).not.toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Show Grafi message" }));
    expect(screen.getByText("Add context notes.")).toBeInTheDocument();
  });
});

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
