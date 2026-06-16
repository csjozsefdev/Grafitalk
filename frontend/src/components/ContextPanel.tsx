interface ContextPanelProps {
  value: string;
  onChange: (value: string) => void;
  onImport?: () => void;
  disabled?: boolean;
  importDisabled?: boolean;
  importFeedback?: string | null;
  saveError?: string | null;
}

export function ContextPanel({
  value,
  onChange,
  onImport,
  disabled = false,
  importDisabled = false,
  importFeedback = null,
  saveError = null,
}: ContextPanelProps) {
  return (
    <aside className="gt-context" aria-label="Project context">
      <div className="gt-context__section">
        <div className="gt-context__header">
          <h2 className="gt-context__heading">Context</h2>
          <div className="gt-context__header-actions">
            {importFeedback ? (
              <span className="gt-context__feedback" aria-live="polite">
                {importFeedback}
              </span>
            ) : null}
            <button
              type="button"
              className="gt-btn gt-btn--ghost gt-context__import"
              onClick={onImport}
              disabled={disabled || importDisabled || !onImport}
            >
              Import
            </button>
          </div>
        </div>
        <textarea
          className="gt-context__editor"
          value={value}
          onChange={(event) => onChange(event.target.value)}
          placeholder="Paste notes, handoff details, or session context for this project."
          disabled={disabled}
          aria-label="Project context notes"
        />
      </div>
      {saveError ? (
        <p className="gt-context__error" role="alert">
          {saveError}
        </p>
      ) : null}
    </aside>
  );
}
