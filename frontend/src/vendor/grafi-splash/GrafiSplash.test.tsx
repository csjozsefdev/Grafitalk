import { act, cleanup, render } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { GrafiSplash } from "./GrafiSplash";

vi.mock("./assets/grafi-splash.png", () => ({
  default: "grafi-splash-head-mock.png",
}));

describe("GrafiSplash", () => {
  afterEach(() => {
    cleanup();
    vi.useRealTimers();
  });

  beforeEach(() => {
    vi.useFakeTimers({ toFake: ["setTimeout", "setInterval", "Date"] });
  });

  it("renders head image and ring as separate layers", () => {
    render(
      <GrafiSplash
        visible
        message="Waking up"
        imageSrc="head-only.png"
        imageAlt="Grafi head"
        onHidden={vi.fn()}
      />
    );

    const image = document.querySelector(".grafi-splash__image");
    const ring = document.querySelector(".grafi-splash__ring-highlight");

    expect(image?.getAttribute("src")).toBe("head-only.png");
    expect(ring).not.toBeNull();
    expect(ring?.parentElement?.classList.contains("grafi-splash__ring-positioner")).toBe(
      true
    );
  });

  it("applies white-background class from host wrapper", () => {
    render(
      <GrafiSplash
        visible
        className="grafi-splash--grafitalk"
        onHidden={vi.fn()}
      />
    );

    const splash = document.querySelector(".grafi-splash--grafitalk");
    expect(splash).not.toBeNull();
  });

  it("dismisses after initialization completes", () => {
    const onHidden = vi.fn();
    const { rerender } = render(
      <GrafiSplash visible message="Loading" onHidden={onHidden} />
    );

    rerender(<GrafiSplash visible={false} message="Loading" onHidden={onHidden} />);

    act(() => {
      vi.advanceTimersByTime(1200);
    });

    act(() => {
      vi.advanceTimersByTime(550);
    });

    expect(onHidden).toHaveBeenCalledTimes(1);
  });

  it("keeps the head image outside the spinning ring layer", () => {
    render(
      <GrafiSplash
        visible
        imageSrc="head-only.png"
        onHidden={vi.fn()}
      />
    );

    const image = document.querySelector(".grafi-splash__image");
    const ring = document.querySelector(".grafi-splash__ring-highlight--spinning");

    expect(image?.classList.contains("grafi-splash__ring-highlight--spinning")).toBe(
      false
    );
    expect(ring).not.toBeNull();
  });

  it("cleans up timers on unmount", () => {
    const clearTimeoutSpy = vi.spyOn(window, "clearTimeout");
    const clearIntervalSpy = vi.spyOn(window, "clearInterval");

    const { unmount } = render(<GrafiSplash visible onHidden={vi.fn()} />);
    unmount();

    expect(clearTimeoutSpy.mock.calls.length + clearIntervalSpy.mock.calls.length).toBeGreaterThan(
      0
    );
  });
});
