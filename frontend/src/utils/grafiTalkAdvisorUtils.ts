import type { GrafiAdvisorMessage } from "../hooks/useGrafiAdvisor";
import { isTransientGrafiMessage } from "../hooks/useGrafiAdvisor";

export function shouldAutoExpandGrafiMessage(
  message: GrafiAdvisorMessage
): boolean {
  if (isTransientGrafiMessage(message)) {
    return true;
  }

  return message.priority === "high" || message.priority === "medium";
}
