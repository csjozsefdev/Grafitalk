import { useCallback, useRef, useState } from "react";

import type { ConfirmDialogTone } from "../components/ConfirmDialog";

export interface ConfirmDialogRequest {
  title: string;
  description: string;
  confirmLabel?: string;
  cancelLabel?: string;
  tone?: ConfirmDialogTone;
}

interface ConfirmDialogState extends ConfirmDialogRequest {
  open: boolean;
}

const CLOSED_STATE: ConfirmDialogState = {
  open: false,
  title: "",
  description: "",
};

export function useConfirmDialog() {
  const [state, setState] = useState<ConfirmDialogState>(CLOSED_STATE);
  const resolverRef = useRef<((confirmed: boolean) => void) | null>(null);

  const close = useCallback((confirmed: boolean) => {
    setState(CLOSED_STATE);
    const resolver = resolverRef.current;
    resolverRef.current = null;
    resolver?.(confirmed);
  }, []);

  const confirm = useCallback((request: ConfirmDialogRequest) => {
    return new Promise<boolean>((resolve) => {
      resolverRef.current = resolve;
      setState({
        open: true,
        ...request,
      });
    });
  }, []);

  const dialogProps = {
    open: state.open,
    title: state.title,
    description: state.description,
    confirmLabel: state.confirmLabel,
    cancelLabel: state.cancelLabel,
    tone: state.tone,
    onConfirm: () => close(true),
    onCancel: () => close(false),
  };

  return { confirm, dialogProps };
}
