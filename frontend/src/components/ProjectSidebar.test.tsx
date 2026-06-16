import { cleanup, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";

import type { Project } from "../types/project";
import { ProjectSidebar } from "./ProjectSidebar";

const sampleProject: Project = {
  id: "p1",
  name: "Client Alpha",
  client_label: null,
  status: "active",
  created_at: "2026-01-01T00:00:00Z",
  updated_at: "2026-01-01T00:00:00Z",
  last_used_at: "2026-06-01T00:00:00Z",
  archived_at: null,
};

const archivedProject: Project = {
  ...sampleProject,
  id: "p2",
  name: "Old Client",
  status: "archived",
  archived_at: "2026-06-10T00:00:00Z",
};

describe("ProjectSidebar", () => {
  afterEach(() => {
    cleanup();
  });

  it("shows the all-archived empty state", () => {
    render(
      <ProjectSidebar
        projects={[]}
        archivedProjects={[archivedProject]}
        showArchived={false}
        onToggleArchived={vi.fn()}
        selectedId=""
        onSelect={vi.fn()}
        onCreate={vi.fn()}
        onArchiveRequest={vi.fn()}
        onRenameRequest={vi.fn()}
        onRestore={vi.fn()}
      />
    );

    expect(
      screen.getByText(/All projects are archived/i)
    ).toBeInTheDocument();
  });

  it("toggles archived section", async () => {
    const user = userEvent.setup();
    const onToggleArchived = vi.fn();

    render(
      <ProjectSidebar
        projects={[sampleProject]}
        archivedProjects={[archivedProject]}
        showArchived={false}
        onToggleArchived={onToggleArchived}
        selectedId="p1"
        onSelect={vi.fn()}
        onCreate={vi.fn()}
        onArchiveRequest={vi.fn()}
        onRenameRequest={vi.fn()}
        onRestore={vi.fn()}
      />
    );

    await user.click(screen.getByRole("button", { name: /Show archived/i }));
    expect(onToggleArchived).toHaveBeenCalledTimes(1);
  });

  it("calls onRestore when restore is clicked", async () => {
    const user = userEvent.setup();
    const onRestore = vi.fn().mockResolvedValue(undefined);

    render(
      <ProjectSidebar
        projects={[]}
        archivedProjects={[archivedProject]}
        showArchived
        onToggleArchived={vi.fn()}
        selectedId=""
        onSelect={vi.fn()}
        onCreate={vi.fn()}
        onArchiveRequest={vi.fn()}
        onRenameRequest={vi.fn()}
        onRestore={onRestore}
      />
    );

    await user.click(screen.getByRole("button", { name: /Restore Old Client/i }));
    expect(onRestore).toHaveBeenCalledWith("p2");
  });

  it("shows action errors", () => {
    render(
      <ProjectSidebar
        projects={[sampleProject]}
        archivedProjects={[]}
        showArchived={false}
        onToggleArchived={vi.fn()}
        selectedId="p1"
        onSelect={vi.fn()}
        onCreate={vi.fn()}
        onArchiveRequest={vi.fn()}
        onRenameRequest={vi.fn()}
        onRestore={vi.fn()}
        actionError="Failed to rename project"
      />
    );

    expect(screen.getByRole("alert")).toHaveTextContent("Failed to rename project");
  });

  it("opens settings from the projects header", async () => {
    const user = userEvent.setup();
    const onOpenSettings = vi.fn();

    render(
      <ProjectSidebar
        projects={[sampleProject]}
        archivedProjects={[]}
        showArchived={false}
        onToggleArchived={vi.fn()}
        selectedId="p1"
        onSelect={vi.fn()}
        onCreate={vi.fn()}
        onArchiveRequest={vi.fn()}
        onRenameRequest={vi.fn()}
        onRestore={vi.fn()}
        onOpenSettings={onOpenSettings}
      />
    );

    await user.click(screen.getByRole("button", { name: /open settings/i }));
    expect(onOpenSettings).toHaveBeenCalledTimes(1);
    expect(screen.queryByRole("button", { name: /^settings$/i })).not.toBeInTheDocument();
  });

  it("shows a compact actions menu on each project card", async () => {
    const user = userEvent.setup();
    const onRenameRequest = vi.fn();

    render(
      <ProjectSidebar
        projects={[sampleProject]}
        archivedProjects={[]}
        showArchived={false}
        onToggleArchived={vi.fn()}
        selectedId="p1"
        onSelect={vi.fn()}
        onCreate={vi.fn()}
        onArchiveRequest={vi.fn()}
        onRenameRequest={onRenameRequest}
        onRestore={vi.fn()}
      />
    );

    expect(
      screen.getByRole("button", { name: /Actions for Client Alpha/i })
    ).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /^Rename$/i })).not.toBeInTheDocument();

    await user.click(
      screen.getByRole("button", { name: /Actions for Client Alpha/i })
    );
    await user.click(screen.getByRole("menuitem", { name: "Rename" }));

    expect(onRenameRequest).toHaveBeenCalledWith(sampleProject);
    expect(screen.queryByRole("menuitem", { name: "Rename" })).not.toBeInTheDocument();
  });

  it("calls onArchiveRequest from the remove menu item", async () => {
    const user = userEvent.setup();
    const onArchiveRequest = vi.fn().mockResolvedValue(undefined);

    render(
      <ProjectSidebar
        projects={[sampleProject]}
        archivedProjects={[]}
        showArchived={false}
        onToggleArchived={vi.fn()}
        selectedId="p1"
        onSelect={vi.fn()}
        onCreate={vi.fn()}
        onArchiveRequest={onArchiveRequest}
        onRenameRequest={vi.fn()}
        onRestore={vi.fn()}
      />
    );

    await user.click(
      screen.getByRole("button", { name: /Actions for Client Alpha/i })
    );
    await user.click(screen.getByRole("menuitem", { name: "Remove" }));

    expect(onArchiveRequest).toHaveBeenCalledWith(sampleProject);
  });
});
