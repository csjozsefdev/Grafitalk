import type { ReactNode } from "react";

interface DocumentPanelProps {
  title: string;
  body: string;
  onBodyChange: (value: string) => void;
  draftActions?: ReactNode;
  outputActions?: ReactNode;
  disabled?: boolean;
  saveError?: string | null;
  draftReady?: boolean;
  contextSaved?: boolean;
  lastGeneratedLabel?: string | null;
  templateLabel?: string | null;
  lastExportedLabel?: string | null;
  readAloudSupported?: boolean;
  readAloudEnabled?: boolean;
  readAloudReadiness?: "unsupported" | "loading" | "ready" | "error";
  readAloudError?: string | null;
  isSpeaking?: boolean;
  onReadAloud?: () => void;
}

export function DocumentPanel({
  title,
  body,
  onBodyChange,
  draftActions,
  outputActions,
  disabled = false,
  saveError = null,
  draftReady = false,
  contextSaved = false,
  lastGeneratedLabel = null,
  templateLabel = null,
  lastExportedLabel = null,
  readAloudSupported = false,
  readAloudEnabled = true,
  readAloudReadiness = "unsupported",
  readAloudError = null,
  isSpeaking = false,
  onReadAloud,
}: DocumentPanelProps) {
  const subjectLine =
    title === "Select a project" ? "Select a project" : title.replace(" — ", " – ");

  const canReadAloud =
    readAloudSupported &&
    readAloudEnabled &&
    readAloudReadiness === "ready" &&
    draftReady &&
    Boolean(onReadAloud);

  const readAloudHint = !readAloudEnabled
    ? "Enable read-aloud in Settings."
    : readAloudReadiness === "unsupported"
      ? "Read-aloud is not available in this environment."
      : readAloudReadiness === "loading"
        ? "Read-aloud is starting…"
        : readAloudError;

  return (
    <section className="gt-document gt-document--hero" aria-label="Document review">
      <div className="gt-document__toolbar">
        <div className="gt-document__toolbar-info">
          <div className="gt-document__label-row">
            <p className="gt-document__label">Current preview</p>
            <div className="gt-document__header-actions">
              {outputActions}
              {readAloudSupported ? (
                <button
                  type="button"
                  className={
                    isSpeaking
                      ? "gt-read-aloud gt-read-aloud--active"
                      : "gt-read-aloud"
                  }
                  onClick={onReadAloud}
                  disabled={!canReadAloud || disabled}
                  aria-label={
                    isSpeaking ? "Stop reading draft aloud" : "Read draft aloud"
                  }
                  title={
                    canReadAloud
                      ? isSpeaking
                        ? "Stop reading"
                        : "Read draft aloud"
                      : readAloudHint ?? "Add draft text to enable read-aloud"
                  }
                >
                  {isSpeaking ? "■" : "🔊"}
                </button>
              ) : (
                <span className="gt-read-aloud-hint" role="status">
                  Read-aloud is not available in this environment.
                </span>
              )}
            </div>
          </div>
          {readAloudSupported && readAloudHint && readAloudEnabled ? (
            <p className="gt-read-aloud-hint" role="status">
              {readAloudHint}
            </p>
          ) : null}
          <p className="gt-document__hint">{title}</p>
          <div className="gt-status-row" aria-label="Draft status">
            <span
              className={
                draftReady ? "gt-status gt-status--ready" : "gt-status gt-status--idle"
              }
            >
              <span className="gt-status__dot" aria-hidden="true" />
              Draft ready
            </span>
            <span
              className={
                contextSaved
                  ? "gt-status gt-status--ready"
                  : "gt-status gt-status--idle"
              }
            >
              <span className="gt-status__dot" aria-hidden="true" />
              Context saved
            </span>
            <span className="gt-status gt-status--meta">
              Last generated: {lastGeneratedLabel ?? "Not yet"}
            </span>
            <span className="gt-status gt-status--meta">
              Template: {templateLabel ?? "Status Update"}
            </span>
            <span className="gt-status gt-status--meta">
              Exported: {lastExportedLabel ?? "Not yet"}
            </span>
          </div>
          {draftActions ? (
            <div className="gt-document__draft-row">{draftActions}</div>
          ) : null}
        </div>
      </div>
      <div className="gt-document__surface">
        <div className="gt-document__email-header">
          <p className="gt-document__preview-label">Current preview</p>
          <p className="gt-document__subject">
            <span className="gt-document__subject-key">Subject:</span>
            <span className="gt-document__subject-value">{subjectLine}</span>
          </p>
        </div>
        <div className="gt-document__email-body">
          <textarea
            className="gt-document__editor"
            value={body}
            onChange={(event) => onBodyChange(event.target.value)}
            aria-label="Draft document"
            spellCheck
            disabled={disabled}
            placeholder="Generate a draft or start writing your client update here."
          />
        </div>
        {saveError ? (
          <p className="gt-document__error" role="alert">
            {saveError}
          </p>
        ) : null}
      </div>
    </section>
  );
}
