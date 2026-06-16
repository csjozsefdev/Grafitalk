import { useEffect, useState } from "react";

import whiteGrafiSplash from "../../vendor/grafi-splash/assets/white-grafi-splash.jpg";
import { GrafiSplash } from "../../vendor/grafi-splash/GrafiSplash";
import {
  GRAFITALK_SPLASH_FADE_OUT_MS,
  GRAFITALK_SPLASH_MESSAGE_INTERVAL_MS,
  GRAFITALK_SPLASH_MESSAGES,
  GRAFITALK_SPLASH_MIN_DURATION_MS,
} from "./grafitalkSplashConfig";
import "./GrafiTalkSplash.css";

export interface GrafiTalkSplashProps {
  /** When false, splash begins minimum-duration hold then fades out. */
  visible: boolean;
  onHidden: () => void;
}

export function GrafiTalkSplash({ visible, onHidden }: GrafiTalkSplashProps) {
  const [loadingMessageIndex, setLoadingMessageIndex] = useState(0);

  useEffect(() => {
    if (!visible) {
      return;
    }

    const timer = window.setInterval(() => {
      setLoadingMessageIndex((current) => {
        const lastLoadingIndex = GRAFITALK_SPLASH_MESSAGES.length - 2;
        return current >= lastLoadingIndex ? current : current + 1;
      });
    }, GRAFITALK_SPLASH_MESSAGE_INTERVAL_MS);

    return () => {
      window.clearInterval(timer);
    };
  }, [visible]);

  const messageIndex = visible
    ? loadingMessageIndex
    : GRAFITALK_SPLASH_MESSAGES.length - 1;
  const message = GRAFITALK_SPLASH_MESSAGES[messageIndex];

  return (
    <GrafiSplash
      className="grafi-splash--grafitalk"
      message={message}
      visible={visible}
      minDurationMs={GRAFITALK_SPLASH_MIN_DURATION_MS}
      fadeOutMs={GRAFITALK_SPLASH_FADE_OUT_MS}
      imageSrc={whiteGrafiSplash}
      imageAlt="GrafiTalk startup logo"
      showLoadingRing={false}
      onHidden={onHidden}
    />
  );
}
