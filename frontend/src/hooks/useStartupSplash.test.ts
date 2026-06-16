import { act, renderHook } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { useStartupSplash } from "./useStartupSplash";

describe("useStartupSplash", () => {
  it("starts with splash mounted and app not ready", () => {
    const { result } = renderHook(() => useStartupSplash());

    expect(result.current.appReady).toBe(false);
    expect(result.current.splashMounted).toBe(true);
  });

  it("marks app ready and unmounts splash after hidden callback", () => {
    const { result } = renderHook(() => useStartupSplash());

    act(() => {
      result.current.markAppReady();
    });
    expect(result.current.appReady).toBe(true);
    expect(result.current.splashMounted).toBe(true);

    act(() => {
      result.current.handleSplashHidden();
    });
    expect(result.current.splashMounted).toBe(false);
  });
});
