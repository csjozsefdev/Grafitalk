import { useCallback, useRef, useState } from "react";

export interface NamePromptRequest {
  title: string;
  description?: string;
  initialValue?: string;
  confirmLabel?: string;
  cancelLabel?: string;
  maxLength?: number;
}

interface NamePromptState extends NamePromptRequest {
  open: boolean;
}

const CLOSED_STATE: NamePromptState = {
  open: false,
  title: "",
};

export function useNamePromptDialog() {
  const [state, setState] = useState<NamePromptState>(CLOSED_STATE);
  const resolverRef = useRef<((value: string | null) => void) | null>(null);

  const close = useCallback((value: string | null) => {
    setState(CLOSED_STATE);
    const resolver = resolverRef.current;
    resolverRef.current = null;
    resolver?.(value);
  }, []);

  const prompt = useCallback((request: NamePromptRequest) => {
    return new Promise<string | null>((resolve) => {
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
    initialValue: state.initialValue,
    confirmLabel: state.confirmLabel,
    cancelLabel: state.cancelLabel,
    maxLength: state.maxLength,
    onConfirm: (value: string) => close(value),
    onCancel: () => close(null),
  };

  return { prompt, dialogProps };
}
