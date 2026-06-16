import { useEffect, useRef } from "react";

export type ConfirmDialogTone = "default" | "destructive";

export interface ConfirmDialogProps {
  open: boolean;
  title: string;
  description: string;
  confirmLabel?: string;
  cancelLabel?: string;
  tone?: ConfirmDialogTone;
  onConfirm: () => void;
  onCancel: () => void;
}

export function ConfirmDialog({
  open,
  title,
  description,
  confirmLabel = "Confirm",
  cancelLabel = "Cancel",
  tone = "default",
  onConfirm,
  onCancel,
}: ConfirmDialogProps) {
  const confirmRef = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    if (!open) {
      return;
    }

    confirmRef.current?.focus();

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

  const confirmClass =
    tone === "destructive"
      ? "gt-btn gt-btn--danger"
      : "gt-btn gt-btn--primary";

  return (
    <div
      className="gt-modal-backdrop"
      role="presentation"
      onClick={onCancel}
    >
      <div
        className="gt-modal gt-confirm-dialog"
        role="alertdialog"
        aria-labelledby="gt-confirm-title"
        aria-describedby="gt-confirm-description"
        onClick={(event) => event.stopPropagation()}
      >
        <div className="gt-modal__header">
          <h2 id="gt-confirm-title">{title}</h2>
        </div>
        <p id="gt-confirm-description" className="gt-confirm-dialog__description">
          {description}
        </p>
        <div className="gt-confirm-dialog__actions">
          <button
            type="button"
            className="gt-btn gt-btn--ghost"
            onClick={onCancel}
          >
            {cancelLabel}
          </button>
          <button
            ref={confirmRef}
            type="button"
            className={confirmClass}
            onClick={onConfirm}
          >
            {confirmLabel}
          </button>
        </div>
      </div>
    </div>
  );
}
