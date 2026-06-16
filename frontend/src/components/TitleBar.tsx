import { getCurrentWindow } from "@tauri-apps/api/window";

import type { WorkflowStep } from "../data/workflow";
import { WORKFLOW_STEPS } from "../data/workflow";
import grafitalkIcon from "../assets/grafitalk-icon.png";

interface TitleBarProps {
  activeStep: WorkflowStep;
  onOpenDiagnostics?: () => void;
}

export function TitleBar({
  activeStep,
  onOpenDiagnostics,
}: TitleBarProps) {
  const activeIndex = WORKFLOW_STEPS.findIndex((step) => step.id === activeStep);
  const appWindow = getCurrentWindow();

  return (
    <header className="gt-titlebar">
      <div className="gt-titlebar__brand" data-tauri-drag-region>
        <img
          className="gt-titlebar__icon"
          src={grafitalkIcon}
          alt=""
          width={36}
          height={36}
        />
        <div className="gt-titlebar__brand-text">
          <span className="gt-titlebar__logo">GrafiTalk</span>
          <span className="gt-titlebar__tagline">
            Communication workbench · Review before sending
          </span>
        </div>
      </div>

      <nav className="gt-workflow" aria-label="Workflow steps" data-tauri-drag-region>
        <div className="gt-workflow__pill">
          {WORKFLOW_STEPS.map((step, index) => {
            const isActive = step.id === activeStep;
            const isCompleted = index < activeIndex;
            const stepClass = isActive
              ? "gt-workflow__step gt-workflow__step--active"
              : isCompleted
                ? "gt-workflow__step gt-workflow__step--completed"
                : "gt-workflow__step gt-workflow__step--upcoming";

            return (
              <span key={step.id} style={{ display: "contents" }}>
                {index > 0 ? (
                  <span className="gt-workflow__sep" aria-hidden="true">
                    →
                  </span>
                ) : null}
                <span className={stepClass}>
                  {isCompleted ? (
                    <span className="gt-workflow__check" aria-hidden="true">
                      ✓
                    </span>
                  ) : (
                    <span className="gt-workflow__index" aria-hidden="true">
                      {index + 1}
                    </span>
                  )}
                  <span className="gt-workflow__label">{step.label}</span>
                </span>
              </span>
            );
          })}
        </div>
      </nav>

      <div className="gt-titlebar__controls">
        {onOpenDiagnostics ? (
          <button
            type="button"
            className="gt-btn gt-btn--ghost gt-titlebar__utility"
            onClick={onOpenDiagnostics}
            aria-label="Open diagnostics"
            title="About and diagnostics"
          >
            i
          </button>
        ) : null}
        <button
          type="button"
          className="gt-titlebar__control"
          onClick={() => void appWindow.minimize()}
          aria-label="Minimize window"
        >
          ─
        </button>
        <button
          type="button"
          className="gt-titlebar__control"
          onClick={() => void appWindow.toggleMaximize()}
          aria-label="Maximize window"
        >
          ▢
        </button>
        <button
          type="button"
          className="gt-titlebar__control gt-titlebar__control--close"
          onClick={() => void appWindow.close()}
          aria-label="Close window"
        >
          ×
        </button>
      </div>
    </header>
  );
}
