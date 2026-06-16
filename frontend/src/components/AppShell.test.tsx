import { cleanup, render, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { AppShell } from "./AppShell";

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => ({
    minimize: vi.fn(),
    toggleMaximize: vi.fn(),
    close: vi.fn(),
    isMaximized: vi.fn().mockResolvedValue(false),
  }),
}));

vi.mock("../api/projects", () => ({
  archiveProject: vi.fn(),
  createProject: vi.fn(),
  listArchivedProjects: vi.fn().mockResolvedValue([]),
  listProjects: vi.fn().mockResolvedValue([]),
  MAX_PROJECT_NAME_LEN: 80,
  renameProject: vi.fn(),
  restoreProject: vi.fn(),
  touchProjectUsed: vi.fn(),
}));

vi.mock("../api/context", () => ({
  getProjectContext: vi.fn().mockResolvedValue(""),
  saveProjectContext: vi.fn(),
}));

vi.mock("../api/draft", () => ({
  generateDraft: vi.fn(),
  getProjectDraft: vi.fn().mockResolvedValue(""),
  saveProjectDraft: vi.fn(),
  saveProjectTemplateKind: vi.fn(),
}));

vi.mock("../api/reviewMetadata", () => ({
  exportFormatFromPath: vi.fn(),
  formatReviewTemplateLabel: vi.fn(() => "Status Update"),
  getProjectReviewMetadata: vi.fn().mockResolvedValue({
    lastGeneratedAt: null,
    lastExportedAt: null,
    lastExportFormat: null,
    lastTemplateKind: "status_update",
  }),
  migrateLastGeneratedAtIfEmpty: vi.fn(),
  recordDraftExport: vi.fn(),
}));

vi.mock("../api/app", () => ({
  exportDatabaseBackup: vi.fn(),
  getAppDiagnostics: vi.fn(),
  openDataFolder: vi.fn(),
  formatDiagnosticsText: vi.fn(() => "diagnostics"),
}));

vi.mock("../api/import", () => ({
  formatHintFromPath: vi.fn(),
  importGrafIdHandoff: vi.fn(),
}));

vi.mock("../api/export", () => ({
  buildSuggestedExportFilename: vi.fn(),
  prepareDraftExport: vi.fn(),
}));

vi.mock("../utils/lastGeneratedStorage", () => ({
  loadLastGeneratedByProject: vi.fn(() => ({})),
}));

vi.mock("../vendor/grafi/assets/grafi-transparent.png", () => ({
  default: "grafi-transparent-mock.png",
}));

describe("AppShell", () => {
  beforeEach(() => {
    window.localStorage.clear();
  });

  afterEach(() => {
    cleanup();
    document.body.innerHTML = "";
    vi.clearAllMocks();
  });

  it("mounts Grafi advisor through a body portal host", async () => {
    render(<AppShell />);

    await waitFor(() => {
      expect(document.body.querySelector(".gt-grafi-host")).not.toBeNull();
    });

    expect(document.body.querySelector(".grafi-advisor--placement-static")).not.toBeNull();
    expect(document.querySelector(".gt-grafi-overlay")).toBeNull();
  });
});
