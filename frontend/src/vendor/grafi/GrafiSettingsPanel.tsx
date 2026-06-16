import { DEFAULT_GRAFI_MESSAGES } from './defaultMessages';
import type { GrafiSettings } from './grafiTypes';

export interface GrafiSettingsPanelProps {
  settings: GrafiSettings;
  onSettingsChange?: (settings: GrafiSettings) => void;
  title?: string;
  hint?: string;
  className?: string;
}

/** Optional settings section for host app settings screens — not part of the floating advisor. */
export function GrafiSettingsPanel({
  settings,
  onSettingsChange,
  title = DEFAULT_GRAFI_MESSAGES.hostSettingsSectionTitle,
  hint,
  className,
}: GrafiSettingsPanelProps) {
  const labels = DEFAULT_GRAFI_MESSAGES.settingsLabels;

  const handleChange = (field: keyof GrafiSettings, value: boolean) => {
    onSettingsChange?.({ ...settings, [field]: value });
  };

  return (
    <section
      className={['grafi-settings', className].filter(Boolean).join(' ')}
      aria-label={title}
    >
      <h3 className="grafi-settings__title">{title}</h3>
      {hint ? <p className="grafi-settings__hint">{hint}</p> : null}
      <ul className="grafi-settings__list">
        {(Object.keys(labels) as Array<keyof GrafiSettings>).map((field) => (
          <li key={field} className="grafi-settings__item">
            <label>
              <input
                type="checkbox"
                checked={settings[field]}
                onChange={(e) => handleChange(field, e.target.checked)}
              />
              {labels[field]}
            </label>
          </li>
        ))}
      </ul>
    </section>
  );
}
