export type WorkflowStep = "context" | "template" | "draft" | "review" | "copy";

export const WORKFLOW_STEPS: { id: WorkflowStep; label: string }[] = [
  { id: "context", label: "Context" },
  { id: "template", label: "Template" },
  { id: "draft", label: "Draft" },
  { id: "review", label: "Review" },
  { id: "copy", label: "Copy" },
];
