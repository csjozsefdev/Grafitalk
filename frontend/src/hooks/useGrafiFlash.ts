import { useCallback, useMemo, useRef, useState } from "react";

import type { GrafiTalkPreferences } from "../types/grafiPreferences";
import {
  grafiEventMessage,
  pickGrafiMessage,
  useGrafiAdvisor,
  type GrafiAdvisorEvent,
  type GrafiAdvisorMessage,
} from "./useGrafiAdvisor";

export interface GrafiFlashInput {
  preferences: GrafiTalkPreferences;
  selectedId: string | null;
  hasContext: boolean;
  draftReady: boolean;
  isDraftDirty: boolean;
  sessionCompletion: "none" | "copied" | "exported";
  activeProjectCount: number;
  allProjectsArchived: boolean;
}

export function useGrafiFlash(input: GrafiFlashInput) {
  const [flashMessage, setFlashMessage] = useState<GrafiAdvisorMessage | null>(
    null
  );
  const flashNonceRef = useRef(0);

  const persistentMessage = useGrafiAdvisor({
    preferences: input.preferences,
    selectedId: input.selectedId,
    hasContext: input.hasContext,
    draftReady: input.draftReady,
    isDraftDirty: input.isDraftDirty,
    sessionCompletion: input.sessionCompletion,
    activeProjectCount: input.activeProjectCount,
    allProjectsArchived: input.allProjectsArchived,
  });

  const message = useMemo(
    () => pickGrafiMessage(flashMessage, persistentMessage, input.preferences),
    [flashMessage, persistentMessage, input.preferences]
  );

  const emitGrafiEvent = useCallback(
    (event: GrafiAdvisorEvent) => {
      if (!input.preferences.enabled) {
        return;
      }

      const next = grafiEventMessage(event);
      if (!next) {
        return;
      }

      flashNonceRef.current += 1;
      setFlashMessage({
        ...next,
        id: `${next.id}::${flashNonceRef.current}`,
      });
    },
    [input.preferences.enabled]
  );

  return {
    message,
    emitGrafiEvent,
  };
}
