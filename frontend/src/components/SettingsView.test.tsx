import { cleanup, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";

import { SettingsView } from "./SettingsView";
import { DEFAULT_GRAFI_TALK_PREFERENCES } from "../types/grafiPreferences";

describe("SettingsView", () => {
  afterEach(() => {
    cleanup();
  });

  const baseProps = {
    preferences: DEFAULT_GRAFI_TALK_PREFERENCES,
    onPreferencesChange: vi.fn(),
    onBack: vi.fn(),
    diagnostics: null,
    onLoadDiagnostics: vi.fn(),
    onBackup: vi.fn(),
    onOpenDataFolder: vi.fn(),
  };

  it("renders category navigation and returns with Back", async () => {
    const user = userEvent.setup();
    const onBack = vi.fn();

    render(<SettingsView {...baseProps} onBack={onBack} />);

    expect(screen.getByRole("heading", { name: /^settings$/i })).toBeInTheDocument();
    expect(screen.getByRole("navigation", { name: /settings categories/i })).toBeInTheDocument();
    expect(screen.getByText("Export")).toBeInTheDocument();
    expect(screen.getByText("About / Diagnostics")).toBeInTheDocument();
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: /back to workbench/i }));
    expect(onBack).toHaveBeenCalledTimes(1);
  });

  it("updates Grafi and read-aloud preferences", async () => {
    const user = userEvent.setup();
    const onPreferencesChange = vi.fn();

    render(
      <SettingsView {...baseProps} onPreferencesChange={onPreferencesChange} />
    );

    await user.click(screen.getByRole("button", { name: /^read aloud$/i }));
    expect(screen.getByLabelText(/enable read-aloud/i)).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: /^grafi$/i }));
    await user.click(screen.getByLabelText(/motion enabled/i));
    expect(onPreferencesChange).toHaveBeenCalledWith({
      ...DEFAULT_GRAFI_TALK_PREFERENCES,
      motionEnabled: false,
    });
  });

  it("closes on Escape", async () => {
    const user = userEvent.setup();
    const onBack = vi.fn();

    render(<SettingsView {...baseProps} onBack={onBack} />);

    await user.keyboard("{Escape}");
    expect(onBack).toHaveBeenCalledTimes(1);
  });

  it("shows diagnostics in About / Diagnostics", async () => {
    const user = userEvent.setup();

    render(
      <SettingsView
        {...baseProps}
        diagnostics={{
          appVersion: "1.0.0",
          dbPath: "C:\\data\\grafitalk.db",
          projectCount: 2,
          latestMigration: 5,
          platform: "windows",
        }}
      />
    );

    await user.click(screen.getByRole("button", { name: /about \/ diagnostics/i }));
    expect(screen.getByText(/GrafiTalk/i)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /export database backup/i })).toBeInTheDocument();
  });
});
