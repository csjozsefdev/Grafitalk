import { useCallback, useState } from "react";

export interface StartupSplashState {
  appReady: boolean;
  splashMounted: boolean;
  markAppReady: () => void;
  handleSplashHidden: () => void;
}

/** Keeps splash timing out of AppShell business logic. */
export function useStartupSplash(): StartupSplashState {
  const [appReady, setAppReady] = useState(false);
  const [splashMounted, setSplashMounted] = useState(true);

  const markAppReady = useCallback(() => {
    setAppReady(true);
  }, []);

  const handleSplashHidden = useCallback(() => {
    setSplashMounted(false);
  }, []);

  return {
    appReady,
    splashMounted,
    markAppReady,
    handleSplashHidden,
  };
}
