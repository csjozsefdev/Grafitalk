import { useEffect, useState } from "react";
import { createPortal } from "react-dom";

import type { GrafiAdvisorMessage } from "../hooks/useGrafiAdvisor";
import { isTransientGrafiMessage } from "../hooks/useGrafiAdvisor";
import {
  toGrafiSettings,
  type GrafiTalkPreferences,
} from "../types/grafiPreferences";
import { shouldAutoExpandGrafiMessage } from "../utils/grafiTalkAdvisorUtils";
import { GrafiAdvisor } from "../vendor/grafi/GrafiAdvisor";
import type { GrafiDisplayMode, GrafiSeverity } from "../vendor/grafi/grafiTypes";
import "./GrafiTalkAdvisorHost.css";

const TRANSIENT_COLLAPSE_MS = 5500;

interface GrafiTalkAdvisorProps {
  message: GrafiAdvisorMessage | null;
  preferences: GrafiTalkPreferences;
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

function displayModeForMessage(
  message: GrafiAdvisorMessage | null
): GrafiDisplayMode {
  if (!message) {
    return "minimized";
  }

  return shouldAutoExpandGrafiMessage(message) ? "expanded" : "minimized";
}

export function GrafiTalkAdvisor({
  message,
  preferences,
}: GrafiTalkAdvisorProps) {
  const [displayMode, setDisplayMode] = useState<GrafiDisplayMode>("minimized");
  const [trackedMessageId, setTrackedMessageId] = useState<string | null>(null);

  const currentMessageId = message?.id ?? null;

  if (currentMessageId !== trackedMessageId) {
    setTrackedMessageId(currentMessageId);
    setDisplayMode(displayModeForMessage(message));
  }

  useEffect(() => {
    if (!message || !isTransientGrafiMessage(message)) {
      return;
    }

    const timer = window.setTimeout(() => {
      setDisplayMode("minimized");
    }, TRANSIENT_COLLAPSE_MS);

    return () => {
      window.clearTimeout(timer);
    };
  }, [message]);

  const grafiSettings = toGrafiSettings(preferences);

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
