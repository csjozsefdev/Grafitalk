import { useEffect, useState } from "react";

import type { AppDiagnostics } from "../api/app";
import { formatDiagnosticsText } from "../api/app";
import {
  APP_DESCRIPTION,
  APP_NAME,
  APP_VERSION,
} from "../constants/appMeta";
import type { GrafiTalkPreferences } from "../types/grafiPreferences";
import {
  SETTINGS_CATEGORIES,
  type SettingsCategoryId,
} from "../types/settingsCategories";
import "./SettingsView.css";

interface SettingsViewProps {
  preferences: GrafiTalkPreferences;
  onPreferencesChange: (preferences: GrafiTalkPreferences) => void;
  onBack: () => void;
  initialCategory?: SettingsCategoryId;
  diagnostics: AppDiagnostics | null;
  diagnosticsLoading?: boolean;
  diagnosticsFeedback?: string | null;
  diagnosticsBusy?: boolean;
  onLoadDiagnostics: () => void;
  onBackup: () => void;
  onOpenDataFolder: () => void;
}

export function SettingsView({
  preferences,
  onPreferencesChange,
  onBack,
  initialCategory = "general",
  diagnostics,
  diagnosticsLoading = false,
  diagnosticsFeedback = null,
  diagnosticsBusy = false,
  onLoadDiagnostics,
  onBackup,
  onOpenDataFolder,
}: SettingsViewProps) {
  const [activeCategory, setActiveCategory] =
    useState<SettingsCategoryId>(initialCategory);

  useEffect(() => {
    function handleKeyDown(event: KeyboardEvent) {
      if (event.key === "Escape") {
        event.preventDefault();
        onBack();
      }
    }

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [onBack]);

  useEffect(() => {
    if (activeCategory === "about" && !diagnostics && !diagnosticsLoading) {
      onLoadDiagnostics();
    }
  }, [activeCategory, diagnostics, diagnosticsLoading, onLoadDiagnostics]);

  function patch(partial: Partial<GrafiTalkPreferences>) {
    onPreferencesChange({ ...preferences, ...partial });
  }

  const diagnosticsText = diagnostics
    ? formatDiagnosticsText(diagnostics)
    : diagnosticsLoading
      ? "Loading diagnostics…"
      : "Diagnostics not loaded yet.";

  return (
    <section
      className="gt-settings-workspace"
      aria-label="Settings"
      data-testid="grafitalk-settings-view"
    >
      <header className="gt-settings-workspace__header">
        <button
          type="button"
          className="gt-btn gt-btn--ghost gt-settings-workspace__back"
          onClick={onBack}
        >
          ← Back to workbench
        </button>
        <h1 className="gt-settings-workspace__title">Settings</h1>
      </header>

      <div className="gt-settings-workspace__layout">
        <nav className="gt-settings-nav" aria-label="Settings categories">
          {SETTINGS_CATEGORIES.map((category) => {
            const isActive = category.id === activeCategory;
            return (
              <button
                key={category.id}
                type="button"
                className={
                  isActive
                    ? "gt-settings-nav__button gt-settings-nav__button--active"
                    : "gt-settings-nav__button"
                }
                aria-current={isActive ? "page" : undefined}
                onClick={() => setActiveCategory(category.id)}
              >
                {category.label}
              </button>
            );
          })}
        </nav>

        <div className="gt-settings-content">
          {activeCategory === "general" ? (
            <article className="gt-settings-card" aria-labelledby="gt-settings-general">
              <h2 id="gt-settings-general" className="gt-settings-card__title">
                General
              </h2>
              <p className="gt-settings-card__lede">
                Workspace preferences for GrafiTalk.
              </p>
              <dl className="gt-settings-meta">
                <div className="gt-settings-meta__row">
                  <dt>App</dt>
                  <dd>{APP_NAME}</dd>
                </div>
                <div className="gt-settings-meta__row">
                  <dt>Description</dt>
                  <dd>{APP_DESCRIPTION}</dd>
                </div>
                <div className="gt-settings-meta__row">
                  <dt>Version</dt>
                  <dd>{APP_VERSION}</dd>
                </div>
              </dl>
            </article>
          ) : null}

          {activeCategory === "read-aloud" ? (
            <article className="gt-settings-card" aria-labelledby="gt-settings-read-aloud">
              <h2 id="gt-settings-read-aloud" className="gt-settings-card__title">
                Read Aloud
              </h2>
              <p className="gt-settings-card__lede">
                Control draft read-aloud in the review workspace.
              </p>
              <label className="gt-settings-toggle">
                <input
                  type="checkbox"
                  checked={preferences.readAloudEnabled}
                  onChange={(event) =>
                    patch({ readAloudEnabled: event.target.checked })
                  }
                />
                <span>Enable read-aloud</span>
              </label>
              <label className="gt-settings-field" htmlFor="gt-settings-voice">
                <span className="gt-settings-field__label">Voice</span>
                <select
                  id="gt-settings-voice"
                  className="gt-settings-field__select"
                  value={preferences.voiceSelection}
                  onChange={(event) =>
                    patch({
                      voiceSelection:
                        event.target.value === "system-default"
                          ? "system-default"
                          : preferences.voiceSelection,
                    })
                  }
                >
                  <option value="system-default">System default</option>
                  <option value="coming-later" disabled>
                    Voice selection coming later
                  </option>
                </select>
              </label>
            </article>
          ) : null}

          {activeCategory === "grafi" ? (
            <article className="gt-settings-card" aria-labelledby="gt-settings-grafi">
              <h2 id="gt-settings-grafi" className="gt-settings-card__title">
                Grafi
              </h2>
              <p className="gt-settings-card__lede">
                Configure the Grafi advisor for workflow guidance.
              </p>
              <label className="gt-settings-toggle">
                <input
                  type="checkbox"
                  checked={preferences.enabled}
                  onChange={(event) => patch({ enabled: event.target.checked })}
                />
                <span>Enable Grafi Advisor</span>
              </label>
              <label className="gt-settings-toggle">
                <input
                  type="checkbox"
                  checked={preferences.motionEnabled}
                  disabled={!preferences.enabled}
                  onChange={(event) =>
                    patch({ motionEnabled: event.target.checked })
                  }
                />
                <span>Motion enabled</span>
              </label>
              <label className="gt-settings-toggle">
                <input
                  type="checkbox"
                  checked={preferences.criticalAlertsOnly}
                  disabled={!preferences.enabled}
                  onChange={(event) =>
                    patch({ criticalAlertsOnly: event.target.checked })
                  }
                />
                <span>Critical alerts only</span>
              </label>
              <label className="gt-settings-toggle">
                <input
                  type="checkbox"
                  checked={preferences.silentMode}
                  disabled={!preferences.enabled}
                  onChange={(event) =>
                    patch({ silentMode: event.target.checked })
                  }
                />
                <span>Silent mode</span>
              </label>
            </article>
          ) : null}

          {activeCategory === "export" ? (
            <article className="gt-settings-card" aria-labelledby="gt-settings-export">
              <h2 id="gt-settings-export" className="gt-settings-card__title">
                Export
              </h2>
              <p className="gt-settings-card__lede">
                Export defaults and formats will be configured here in a future
                update.
              </p>
              <p className="gt-settings-placeholder">
                For now, use Export in the workbench to save drafts through the
                system file dialog.
              </p>
            </article>
          ) : null}

          {activeCategory === "about" ? (
            <article className="gt-settings-card" aria-labelledby="gt-settings-about">
              <h2 id="gt-settings-about" className="gt-settings-card__title">
                About / Diagnostics
              </h2>
              <p className="gt-settings-card__lede">
                Application diagnostics, backup, and data folder access.
              </p>
              <pre className="gt-settings-diagnostics">{diagnosticsText}</pre>
              <p className="gt-settings-note">
                For backup and manual restore steps, see BACKUP_RESTORE.md in
                the project documentation.
              </p>
              <div className="gt-settings-actions">
                <button
                  type="button"
                  className="gt-btn gt-btn--secondary"
                  onClick={onBackup}
                  disabled={diagnosticsBusy}
                >
                  Export database backup
                </button>
                <button
                  type="button"
                  className="gt-btn gt-btn--secondary"
                  onClick={onOpenDataFolder}
                  disabled={diagnosticsBusy}
                >
                  Show data folder path
                </button>
                <button
                  type="button"
                  className="gt-btn gt-btn--ghost"
                  onClick={() => {
                    if (diagnostics) {
                      void navigator.clipboard.writeText(
                        formatDiagnosticsText(diagnostics)
                      );
                    }
                  }}
                  disabled={!diagnostics}
                >
                  Copy diagnostics
                </button>
              </div>
              {diagnosticsFeedback ? (
                <p className="gt-settings-feedback" role="status">
                  {diagnosticsFeedback}
                </p>
              ) : null}
            </article>
          ) : null}
        </div>
      </div>
    </section>
  );
}
