import { act, cleanup, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { GRAFITALK_SPLASH_MESSAGES } from "./grafitalkSplashConfig";
import { GrafiTalkSplash } from "./GrafiTalkSplash";

vi.mock("../../vendor/grafi-splash/assets/white-grafi-splash.jpg", () => ({
  default: "white-grafi-splash-mock.jpg",
}));

describe("GrafiTalkSplash", () => {
  afterEach(() => {
    cleanup();
    vi.useRealTimers();
  });

  beforeEach(() => {
    vi.useFakeTimers({ toFake: ["setTimeout", "setInterval", "Date"] });
  });

  it("renders with GrafiTalk light theme class and startup copy", () => {
    render(<GrafiTalkSplash visible onHidden={vi.fn()} />);

    const splash = document.querySelector(".grafi-splash--grafitalk");
    expect(splash).not.toBeNull();
    expect(splash?.classList.contains("grafi-splash")).toBe(true);
    expect(
      screen.getByLabelText(GRAFITALK_SPLASH_MESSAGES[0])
    ).toBeInTheDocument();

    const img = document.querySelector(".grafi-splash__image");
    expect(img?.getAttribute("src") ?? "").toContain("white-grafi-splash");

    expect(document.querySelector(".grafi-splash__ring-highlight")).toBeNull();
  });

  it("shows final message when app becomes ready", () => {
    const { rerender } = render(
      <GrafiTalkSplash visible onHidden={vi.fn()} />
    );

    rerender(<GrafiTalkSplash visible={false} onHidden={vi.fn()} />);

    const finalMessage =
      GRAFITALK_SPLASH_MESSAGES[GRAFITALK_SPLASH_MESSAGES.length - 1];
    expect(screen.getByLabelText(finalMessage)).toBeInTheDocument();
  });

  it("calls onHidden after minimum duration and fade-out", () => {
    const onHidden = vi.fn();

    const { rerender } = render(
      <GrafiTalkSplash visible onHidden={onHidden} />
    );

    rerender(<GrafiTalkSplash visible={false} onHidden={onHidden} />);

    expect(onHidden).not.toHaveBeenCalled();

    act(() => {
      vi.advanceTimersByTime(1200);
    });

    const overlay = document.querySelector(".grafi-splash--grafitalk");
    expect(overlay?.classList.contains("grafi-splash--fading")).toBe(true);

    act(() => {
      vi.advanceTimersByTime(550);
    });

    expect(onHidden).toHaveBeenCalledTimes(1);
  });
});
