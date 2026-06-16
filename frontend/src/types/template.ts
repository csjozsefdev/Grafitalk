export type TemplateKind =
  | "status_update"
  | "client_update"
  | "debug_report"
  | "handover"
  | "weekly_summary";

export const DEFAULT_TEMPLATE_KIND: TemplateKind = "status_update";

export const TEMPLATE_OPTIONS: ReadonlyArray<{
  value: TemplateKind;
  label: string;
}> = [
  { value: "status_update", label: "Status Update" },
  { value: "client_update", label: "Client Update" },
  { value: "debug_report", label: "Debug Report" },
  { value: "handover", label: "Handover" },
  { value: "weekly_summary", label: "Weekly Summary" },
];

export function templateTitle(kind: TemplateKind): string {
  const match = TEMPLATE_OPTIONS.find((option) => option.value === kind);
  return match?.label ?? "Status Update";
}

export function isTemplateKind(value: string): value is TemplateKind {
  return TEMPLATE_OPTIONS.some((option) => option.value === value);
}
