import { useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

import type { GrafiAdvisorMessage } from "../hooks/useGrafiAdvisor";
import { isTransientGrafiMessage } from "../hooks/useGrafiAdvisor";
import {
  toGrafiSettings,
  type GrafiTalkPreferences,
} from "../types/grafiPreferences";
import { GrafiAdvisor } from "../vendor/grafi/GrafiAdvisor";
import type { GrafiDisplayMode, GrafiSeverity } from "../vendor/grafi/grafiTypes";
import "./GrafiTalkAdvisorHost.css";

const TRANSIENT_COLLAPSE_MS = 5500;

interface GrafiTalkAdvisorProps {
  message: GrafiAdvisorMessage | null;
  preferences: GrafiTalkPreferences;
}

export function shouldAutoExpandGrafiMessage(
  message: GrafiAdvisorMessage
): boolean {
  if (isTransientGrafiMessage(message)) {
    return true;
  }

  return message.priority === "high" || message.priority === "medium";
}

function messageToSeverity(message: GrafiAdvisorMessage): GrafiSeverity {
  const baseId = message.id.split("::")[0] ?? message.id;

  if (baseId === "export-failed" || message.priority === "high") {
    return "critical";
  }

  if (
    baseId === "copy-success" ||
    baseId === "export-success" ||
    baseId === "import-success" ||
    baseId === "copy-success-persistent" ||
    baseId === "export-success-persistent"
  ) {
    return "success";
  }

  if (
    baseId === "template-overwrite" ||
    baseId === "draft-edited" ||
    baseId === "empty-context" ||
    baseId === "all-archived" ||
    message.priority === "medium"
  ) {
    return "warning";
  }

  return "info";
}

export function GrafiTalkAdvisor({
  message,
  preferences,
}: GrafiTalkAdvisorProps) {
  const [displayMode, setDisplayMode] = useState<GrafiDisplayMode>("minimized");
  const lastMessageIdRef = useRef<string | null>(null);
  const collapseTimerRef = useRef<number | null>(null);

  const grafiSettings = toGrafiSettings(preferences);

  useEffect(() => {
    if (!message) {
      lastMessageIdRef.current = null;
      setDisplayMode("minimized");
      return;
    }

    if (message.id === lastMessageIdRef.current) {
      return;
    }

    lastMessageIdRef.current = message.id;
    setDisplayMode(
      shouldAutoExpandGrafiMessage(message) ? "expanded" : "minimized"
    );

    if (collapseTimerRef.current !== null) {
      window.clearTimeout(collapseTimerRef.current);
      collapseTimerRef.current = null;
    }

    if (!isTransientGrafiMessage(message)) {
      return;
    }

    collapseTimerRef.current = window.setTimeout(() => {
      setDisplayMode("minimized");
      collapseTimerRef.current = null;
    }, TRANSIENT_COLLAPSE_MS);

    return () => {
      if (collapseTimerRef.current !== null) {
        window.clearTimeout(collapseTimerRef.current);
        collapseTimerRef.current = null;
      }
    };
  }, [message]);

  if (!grafiSettings.enabled) {
    return null;
  }

  const severity = message ? messageToSeverity(message) : "info";
  let displayMessage = message?.text ?? null;

  if (
    message &&
    grafiSettings.criticalAlertsOnly &&
    severity !== "critical"
  ) {
    displayMessage = null;
  }

  return createPortal(
    <div className="gt-grafi-host" data-testid="grafitalk-grafi-host">
      <GrafiAdvisor
        appName="GrafiTalk"
        severity={severity}
        message={displayMessage}
        settings={grafiSettings}
        displayMode={displayMode}
        placement="static"
        onDisplayModeChange={setDisplayMode}
        onDismiss={() => {
          setDisplayMode("minimized");
        }}
      />
    </div>,
    document.body
  );
}
