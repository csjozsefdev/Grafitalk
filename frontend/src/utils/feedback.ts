const FEEDBACK_MS = 2200;

export function showFeedback(
  setMessage: (value: string | null) => void,
  message: string,
  durationMs = FEEDBACK_MS
) {
  setMessage(message);
  window.setTimeout(() => setMessage(null), durationMs);
}

export const ERROR_FEEDBACK_MS = 3200;
export const SESSION_COMPLETION_MS = 8000;
