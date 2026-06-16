import { useEffect, useRef } from 'react';
import { DEFAULT_GRAFI_MESSAGES } from './defaultMessages';
import { GrafiBubble } from './GrafiBubble';
import { isMessageVisible } from './messageVisibility';
import type { GrafiAdvisorProps } from './grafiTypes';
import { usePrefersReducedMotion } from './usePrefersReducedMotion';
import './grafi.css';

function getSoundKey(severity: string, message: GrafiAdvisorProps['message']): string {
  const messagePart = typeof message === 'string' ? message : 'node';
  return `${severity}::${messagePart}`;
}

export function GrafiAdvisor({
  appName,
  context,
  severity,
  message,
  actions,
  settings,
  displayMode = 'minimized',
  onDisplayModeChange,
  onDismiss,
  onAction,
  onSoundRequest,
  className,
  placement = 'bottom-left',
}: GrafiAdvisorProps) {
  const lastSoundKeyRef = useRef<string | null>(null);
  const prefersReducedMotion = usePrefersReducedMotion();
  const hasMessage = isMessageVisible(message);

  const isVisible =
    settings.enabled && !(settings.criticalAlertsOnly && severity !== 'critical');

  const motionAllowed = settings.motionEnabled && !prefersReducedMotion;
  const showBubble = displayMode === 'expanded' && hasMessage;

  useEffect(() => {
    if (!isVisible || !hasMessage) {
      return;
    }

    if (!settings.soundEnabled || settings.silentMode) {
      return;
    }

    const soundKey = getSoundKey(severity, message);
    if (lastSoundKeyRef.current === soundKey) {
      return;
    }

    lastSoundKeyRef.current = soundKey;
    onSoundRequest?.(severity);
  }, [
    isVisible,
    hasMessage,
    severity,
    message,
    settings.soundEnabled,
    settings.silentMode,
    onSoundRequest,
  ]);

  if (!isVisible) {
    return null;
  }

  const handleDismiss = () => {
    if (onDismiss) {
      onDismiss();
    } else {
      onDisplayModeChange?.('minimized');
    }
  };

  const handleExpand = () => {
    if (hasMessage) {
      onDisplayModeChange?.('expanded');
    }
  };

  const rootClassName = [
    'grafi-advisor',
    `grafi-advisor--${severity}`,
    `grafi-advisor--placement-${placement}`,
    showBubble ? 'grafi-advisor--expanded' : 'grafi-advisor--minimized',
    motionAllowed ? 'grafi-advisor--motion' : 'grafi-advisor--motion-off',
    className,
  ]
    .filter(Boolean)
    .join(' ');

  return (
    <section className={rootClassName} aria-label={`${DEFAULT_GRAFI_MESSAGES.advisorLabel} — ${appName}`}>
      <GrafiBubble
        severity={severity}
        message={message}
        context={context}
        actions={actions}
        displayMode={displayMode}
        motionEnabled={motionAllowed}
        onAction={onAction}
        onDismiss={handleDismiss}
        onExpand={handleExpand}
      />
    </section>
  );
}
