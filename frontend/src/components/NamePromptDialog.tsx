import { useEffect, useRef } from "react";

export interface NamePromptDialogProps {
  open: boolean;
  title: string;
  description?: string;
  initialValue?: string;
  confirmLabel?: string;
  cancelLabel?: string;
  maxLength?: number;
  onConfirm: (value: string) => void;
  onCancel: () => void;
}

export function NamePromptDialog({
  open,
  title,
  description,
  initialValue = "",
  confirmLabel = "Save",
  cancelLabel = "Cancel",
  maxLength = 120,
  onConfirm,
  onCancel,
}: NamePromptDialogProps) {
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (!open) {
      return;
    }

    inputRef.current?.focus();
    inputRef.current?.select();

    function handleKeyDown(event: KeyboardEvent) {
      if (event.key === "Escape") {
        event.preventDefault();
        onCancel();
      }
    }

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [open, onCancel]);

  if (!open) {
    return null;
  }

  return (
    <div className="gt-modal-backdrop" role="presentation" onClick={onCancel}>
      <div
        className="gt-modal gt-name-prompt-dialog"
        role="dialog"
        aria-labelledby="gt-name-prompt-title"
        onClick={(event) => event.stopPropagation()}
      >
        <div className="gt-modal__header">
          <h2 id="gt-name-prompt-title">{title}</h2>
        </div>
        {description ? (
          <p className="gt-confirm-dialog__description">{description}</p>
        ) : null}
        <form
          key={initialValue}
          className="gt-name-prompt-dialog__form"
          onSubmit={(event) => {
            event.preventDefault();
            const trimmed = (inputRef.current?.value ?? "").trim();
            if (trimmed.length > 0 && trimmed.length <= maxLength) {
              onConfirm(trimmed);
            }
          }}
        >
          <input
            ref={inputRef}
            type="text"
            className="gt-sidebar__input"
            defaultValue={initialValue}
            maxLength={maxLength}
            aria-label={title}
          />
          <div className="gt-confirm-dialog__actions">
            <button
              type="button"
              className="gt-btn gt-btn--ghost"
              onClick={onCancel}
            >
              {cancelLabel}
            </button>
            <button type="submit" className="gt-btn gt-btn--primary">
              {confirmLabel}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
