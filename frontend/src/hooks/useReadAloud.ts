import { useCallback, useEffect, useRef, useState } from "react";

export type ReadAloudReadiness =
  | "unsupported"
  | "loading"
  | "ready"
  | "error";

export function canUseReadAloud(): boolean {
  return (
    typeof window !== "undefined" &&
    "speechSynthesis" in window &&
    "SpeechSynthesisUtterance" in window
  );
}

const VOICE_READY_TIMEOUT_MS = 1500;
const SPEAK_WATCHDOG_MS = 2500;

export function useReadAloud(enabled: boolean) {
  const [isSpeaking, setIsSpeaking] = useState(false);
  const [readiness, setReadiness] = useState<ReadAloudReadiness>(() =>
    canUseReadAloud() ? "loading" : "unsupported"
  );
  const [lastError, setLastError] = useState<string | null>(null);
  const utteranceRef = useRef<SpeechSynthesisUtterance | null>(null);
  const watchdogRef = useRef<number | null>(null);

  const clearWatchdog = useCallback(() => {
    if (watchdogRef.current !== null) {
      window.clearTimeout(watchdogRef.current);
      watchdogRef.current = null;
    }
  }, []);

  const stop = useCallback(() => {
    clearWatchdog();
    if (!canUseReadAloud()) {
      setIsSpeaking(false);
      return;
    }

    window.speechSynthesis.cancel();
    utteranceRef.current = null;
    setIsSpeaking(false);
  }, [clearWatchdog]);

  useEffect(() => {
    if (!canUseReadAloud()) {
      return;
    }

    let cancelled = false;

    function markReady() {
      if (!cancelled) {
        setReadiness("ready");
      }
    }

    function handleVoicesChanged() {
      markReady();
    }

    window.speechSynthesis.addEventListener("voiceschanged", handleVoicesChanged);
    const timer = window.setTimeout(markReady, VOICE_READY_TIMEOUT_MS);
    if (window.speechSynthesis.getVoices().length > 0) {
      markReady();
    }

    return () => {
      cancelled = true;
      window.clearTimeout(timer);
      window.speechSynthesis.removeEventListener(
        "voiceschanged",
        handleVoicesChanged
      );
    };
  }, []);

  const speak = useCallback(
    (text: string) => {
      if (!enabled) {
        return;
      }

      if (!canUseReadAloud()) {
        setReadiness("unsupported");
        setLastError("Read-aloud is not available in this environment.");
        return;
      }

      if (readiness === "loading") {
        setLastError("Read-aloud is still starting. Try again in a moment.");
        return;
      }

      const trimmed = text.trim();
      if (!trimmed) {
        return;
      }

      stop();
      setLastError(null);

      const utterance = new SpeechSynthesisUtterance(trimmed);
      utterance.onstart = () => {
        clearWatchdog();
        setIsSpeaking(true);
      };
      utterance.onend = () => {
        clearWatchdog();
        setIsSpeaking(false);
        utteranceRef.current = null;
      };
      utterance.onerror = () => {
        clearWatchdog();
        setIsSpeaking(false);
        utteranceRef.current = null;
        setReadiness("error");
        setLastError("Read-aloud could not play this draft.");
      };

      utteranceRef.current = utterance;
      setIsSpeaking(true);
      window.speechSynthesis.speak(utterance);

      watchdogRef.current = window.setTimeout(() => {
        if (utteranceRef.current === utterance && !window.speechSynthesis.speaking) {
          setIsSpeaking(false);
          utteranceRef.current = null;
          setReadiness("error");
          setLastError(
            "Read-aloud did not produce audio. It may be unavailable in this environment."
          );
        }
      }, SPEAK_WATCHDOG_MS);
    },
    [clearWatchdog, enabled, readiness, stop]
  );

  const toggle = useCallback(
    (text: string) => {
      if (isSpeaking) {
        stop();
        return;
      }

      speak(text);
    },
    [isSpeaking, speak, stop]
  );

  useEffect(() => () => stop(), [stop]);

  return {
    supported: canUseReadAloud(),
    readiness,
    lastError,
    isSpeaking,
    speak,
    stop,
    toggle,
  };
}
