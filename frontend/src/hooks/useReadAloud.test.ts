import { act, renderHook } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { useReadAloud } from "./useReadAloud";

class MockSpeechSynthesisUtterance {
  text: string;
  onstart: (() => void) | null = null;
  onend: (() => void) | null = null;
  onerror: (() => void) | null = null;

  constructor(text: string) {
    this.text = text;
  }
}

function installSpeechMocks() {
  const speak = vi.fn((utterance: MockSpeechSynthesisUtterance) => {
    utterance.onstart?.();
  });

  Object.defineProperty(window, "speechSynthesis", {
    configurable: true,
    value: {
      speak,
      cancel: vi.fn(),
      getVoices: () => [{ name: "Test Voice" }],
      speaking: false,
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
    },
  });

  Object.defineProperty(window, "SpeechSynthesisUtterance", {
    configurable: true,
    value: MockSpeechSynthesisUtterance,
  });

  return { speak };
}

describe("useReadAloud", () => {
  afterEach(() => {
    vi.useRealTimers();
    vi.unstubAllGlobals();
  });

  it("reports unsupported when speech APIs are missing", () => {
  vi.stubGlobal("speechSynthesis", undefined);

    const { result } = renderHook(() => useReadAloud(true));
    expect(result.current.supported).toBe(false);
    expect(result.current.readiness).toBe("unsupported");
  });

  it("does not speak when draft text is empty", () => {
    const { speak } = installSpeechMocks();
    const { result } = renderHook(() => useReadAloud(true));

    act(() => {
      result.current.speak("   ");
    });

    expect(speak).not.toHaveBeenCalled();
  });

  it("starts and stops speaking", () => {
    installSpeechMocks();
    const { result } = renderHook(() => useReadAloud(true));

    act(() => {
      result.current.speak("Hello client");
    });
    expect(result.current.isSpeaking).toBe(true);

    act(() => {
      result.current.stop();
    });
    expect(result.current.isSpeaking).toBe(false);
  });

  it("cleans up on unmount", () => {
    const { speak } = installSpeechMocks();
    const { result, unmount } = renderHook(() => useReadAloud(true));

    act(() => {
      result.current.speak("Cleanup test");
    });

    unmount();
    expect(window.speechSynthesis.cancel).toHaveBeenCalled();
    expect(speak).toHaveBeenCalled();
  });
});
