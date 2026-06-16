import type { TemplateKind } from "../types/template";

import { TEMPLATE_OPTIONS } from "../types/template";

interface DraftActionBarProps {
  templateKind: TemplateKind;
  onTemplateChange: (kind: TemplateKind) => void;
  onGenerate: () => void;
  generateFeedback: string | null;
  generateDisabled?: boolean;
  templateDisabled?: boolean;
  isGenerating?: boolean;
  isExporting?: boolean;
}

export function DraftActionBar({
  templateKind,
  onTemplateChange,
  onGenerate,
  generateFeedback,
  generateDisabled = false,
  templateDisabled = false,
  isGenerating = false,
  isExporting = false,
}: DraftActionBarProps) {
  return (
    <div className="gt-actions gt-actions--draft">
      <label className="gt-actions__template">
        <span className="gt-actions__template-label">Template</span>
        <select
          className="gt-actions__template-select"
          value={templateKind}
          onChange={(event) =>
            onTemplateChange(event.target.value as TemplateKind)
          }
          disabled={templateDisabled || isGenerating || isExporting}
          aria-label="Draft template"
        >
          {TEMPLATE_OPTIONS.map((option) => (
            <option key={option.value} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </label>

      <button
        type="button"
        className="gt-btn gt-btn--primary"
        onClick={onGenerate}
        disabled={generateDisabled || isGenerating || isExporting}
      >
        {isGenerating ? "Generating…" : "Generate Draft"}
      </button>

      {generateFeedback ? (
        <span className="gt-toast">{generateFeedback}</span>
      ) : null}
    </div>
  );
}

interface OutputActionBarProps {
  onCopy: () => void;
  onExport: () => void;
  copyFeedback: string | null;
  exportFeedback?: string | null;
  copyDisabled?: boolean;
  exportDisabled?: boolean;
  isExporting?: boolean;
}

export function OutputActionBar({
  onCopy,
  onExport,
  copyFeedback,
  exportFeedback = null,
  copyDisabled = false,
  exportDisabled = false,
  isExporting = false,
}: OutputActionBarProps) {
  return (
    <div className="gt-actions gt-actions--output">
      <button
        type="button"
        className="gt-btn gt-btn--primary"
        onClick={onCopy}
        disabled={copyDisabled || isExporting}
      >
        Copy
      </button>
      <button
        type="button"
        className="gt-btn gt-btn--primary"
        onClick={onExport}
        disabled={exportDisabled || isExporting}
      >
        {isExporting ? "Exporting…" : "Export"}
      </button>
      {copyFeedback ? <span className="gt-toast gt-toast--inline">{copyFeedback}</span> : null}
      {exportFeedback ? (
        <span className="gt-toast gt-toast--inline">{exportFeedback}</span>
      ) : null}
    </div>
  );
}
