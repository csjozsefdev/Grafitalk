import type { WorkflowStep } from "../data/workflow";

export type SessionCompletion = "none" | "copied" | "exported";

export interface WorkflowStepInput {
  selectedId: string | null;
  hasContext: boolean;
  draftReady: boolean;
  isGenerating: boolean;
  sessionCompletion: SessionCompletion;
}

export function deriveWorkflowStep(input: WorkflowStepInput): WorkflowStep {
  const {
    selectedId,
    hasContext,
    draftReady,
    isGenerating,
    sessionCompletion,
  } = input;

  if (!selectedId || !hasContext) {
    return "context";
  }

  if (isGenerating) {
    return "draft";
  }

  if (!draftReady) {
    return "template";
  }

  if (sessionCompletion === "copied" || sessionCompletion === "exported") {
    return "copy";
  }

  return "review";
}
