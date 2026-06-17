import { useMemo } from "react";

import type { GrafiTalkPreferences } from "../types/grafiPreferences";

export type GrafiMessagePriority = "low" | "medium" | "high";

export type GrafiAdvisorEvent =
  | "importSucceeded"
  | "copySucceeded"
  | "exportSucceeded"
  | "exportFailed"
  | "templateOverwriteRisk";

export interface GrafiAdvisorInput {
  preferences: GrafiTalkPreferences;
  selectedId: string | null;
  hasContext: boolean;
  draftReady: boolean;
  isDraftDirty: boolean;
  sessionCompletion: "none" | "copied" | "exported";
  activeProjectCount: number;
  allProjectsArchived: boolean;
}

export interface GrafiAdvisorMessage {
  id: string;
  text: string;
  priority: GrafiMessagePriority;
  transient?: boolean;
}

export function grafiMessagePriorityRank(
  priority: GrafiMessagePriority
): number {
  switch (priority) {
    case "high":
      return 3;
    case "medium":
      return 2;
    case "low":
    default:
      return 1;
  }
}

export function isTransientGrafiMessage(message: GrafiAdvisorMessage): boolean {
  return message.transient === true;
}

export function grafiEventMessage(
  event: GrafiAdvisorEvent
): GrafiAdvisorMessage | null {
  switch (event) {
    case "copySucceeded":
      return {
        id: "copy-success",
        text: "Copied to clipboard.",
        priority: "low",
        transient: true,
      };
    case "exportSucceeded":
      return {
        id: "export-success",
        text: "Draft exported.",
        priority: "low",
        transient: true,
      };
    case "importSucceeded":
      return {
        id: "import-success",
        text: "Context imported.",
        priority: "low",
        transient: true,
      };
    case "exportFailed":
      return {
        id: "export-failed",
        text: "Export failed. Check the file path or permissions.",
        priority: "high",
        transient: true,
      };
    case "templateOverwriteRisk":
      return {
        id: "template-overwrite",
        text: "Changing the template will replace your manual edits.",
        priority: "medium",
        transient: true,
      };
    default:
      return null;
  }
}

export function resolveGrafiAdvisorMessage(
  input: GrafiAdvisorInput
): GrafiAdvisorMessage | null {
  const {
    preferences,
    selectedId,
    hasContext,
    draftReady,
    isDraftDirty,
    sessionCompletion,
    activeProjectCount,
    allProjectsArchived,
  } = input;

  if (!preferences.enabled) {
    return null;
  }

  let message: GrafiAdvisorMessage;

  if (allProjectsArchived && activeProjectCount === 0) {
    message = {
      id: "all-archived",
      text: "All projects are archived. Restore one from the sidebar to continue.",
      priority: "medium",
    };
  } else if (!selectedId) {
    message = {
      id: "welcome",
      text: "Create or select a project to start preparing a client-ready draft.",
      priority: "low",
    };
  } else if (!hasContext) {
    message = {
      id: "empty-context",
      text: "Add context notes or import a Graf-ID handoff before generating a draft.",
      priority: "medium",
    };
  } else if (isDraftDirty) {
    message = {
      id: "draft-edited",
      text: "You edited this draft manually. Review your changes before copying or exporting.",
      priority: "medium",
    };
  } else if (sessionCompletion === "copied") {
    message = {
      id: "copy-success-persistent",
      text: "Copied to clipboard. Paste into your email or chat app when ready.",
      priority: "low",
    };
  } else if (sessionCompletion === "exported") {
    message = {
      id: "export-success-persistent",
      text: "Draft exported. Open the file and review before sending.",
      priority: "low",
    };
  } else if (draftReady) {
    message = {
      id: "draft-ready",
      text: "Your draft is ready. Review it, then copy or export when you are satisfied.",
      priority: "low",
    };
  } else {
    message = {
      id: "has-context",
      text: "Choose a template and generate a draft when your context is complete.",
      priority: "low",
    };
  }

  if (preferences.criticalAlertsOnly && message.priority !== "high") {
    return null;
  }

  return message;
}

export function pickGrafiMessage(
  flashMessage: GrafiAdvisorMessage | null,
  persistentMessage: GrafiAdvisorMessage | null,
  preferences: GrafiTalkPreferences
): GrafiAdvisorMessage | null {
  if (!preferences.enabled) {
    return null;
  }

  if (!flashMessage) {
    return persistentMessage;
  }

  if (!persistentMessage) {
    return flashMessage;
  }

  return grafiMessagePriorityRank(flashMessage.priority) >=
    grafiMessagePriorityRank(persistentMessage.priority)
    ? flashMessage
    : persistentMessage;
}

export function useGrafiAdvisor(
  input: GrafiAdvisorInput
): GrafiAdvisorMessage | null {
  return useMemo(() => resolveGrafiAdvisorMessage(input), [input]);
}
