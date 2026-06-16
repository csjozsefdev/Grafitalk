import { useState } from "react";

import type { Project } from "../types/project";
import { formatLastUsed } from "../utils/formatLastUsed";
import { ProjectCardMenu } from "./ProjectCardMenu";
interface ProjectSidebarProps {
  projects: Project[];
  archivedProjects: Project[];
  showArchived: boolean;
  onToggleArchived: () => void;
  selectedId: string;
  onSelect: (id: string) => void;
  onCreate: (name: string) => Promise<void>;
  onArchiveRequest: (project: Project) => Promise<void>;
  onRenameRequest: (project: Project) => void;
  onRestore: (projectId: string) => Promise<void>;
  isLoading?: boolean;
  error?: string | null;
  actionError?: string | null;
  busy?: boolean;
  onOpenSettings?: () => void;
  settingsActive?: boolean;
}

function formatArchivedAt(value: string | null): string {
  if (!value) {
    return "unknown date";
  }
  return formatLastUsed(value);
}

export function ProjectSidebar({
  projects,
  archivedProjects,
  showArchived,
  onToggleArchived,
  selectedId,
  onSelect,
  onCreate,
  onArchiveRequest,
  onRenameRequest,
  onRestore,
  isLoading = false,
  error = null,
  actionError = null,
  busy = false,
  onOpenSettings,
  settingsActive = false,
}: ProjectSidebarProps) {
  const [newName, setNewName] = useState("");
  const [isCreating, setIsCreating] = useState(false);
  const [createError, setCreateError] = useState<string | null>(null);
  const [archivingId, setArchivingId] = useState<string | null>(null);
  const [restoringId, setRestoringId] = useState<string | null>(null);
  const [openMenuId, setOpenMenuId] = useState<string | null>(null);
  async function handleCreate() {
    const trimmed = newName.trim();
    if (!trimmed) {
      setCreateError("Project name is required");
      return;
    }

    setIsCreating(true);
    setCreateError(null);
    try {
      await onCreate(trimmed);
      setNewName("");
    } catch (err) {
      setCreateError(err instanceof Error ? err.message : "Failed to create project");
    } finally {
      setIsCreating(false);
    }
  }

  async function handleArchive(project: Project) {
    setArchivingId(project.id);
    try {
      await onArchiveRequest(project);
    } finally {
      setArchivingId(null);
    }
  }

  async function handleRestore(projectId: string) {
    setRestoringId(projectId);
    try {
      await onRestore(projectId);
    } finally {
      setRestoringId(null);
    }
  }

  const archivedCount = archivedProjects.length;
  const allArchived = !isLoading && projects.length === 0 && archivedCount > 0;

  return (
    <aside className="gt-sidebar">
      <div className="gt-sidebar__header">
        <h2 className="gt-sidebar__heading">Projects</h2>
        {onOpenSettings ? (
          <button
            type="button"
            className={
              settingsActive
                ? "gt-sidebar__settings gt-sidebar__settings--active"
                : "gt-sidebar__settings"
            }
            onClick={onOpenSettings}
            aria-label="Open settings"
          >
            <span aria-hidden="true">⚙</span>
            Settings
          </button>
        ) : null}
      </div>

      <form
        className="gt-sidebar__create"
        onSubmit={(event) => {
          event.preventDefault();
          void handleCreate();
        }}
      >
        <input
          type="text"
          className="gt-sidebar__input"
          placeholder="New project name"
          value={newName}
          onChange={(event) => setNewName(event.target.value)}
          disabled={isCreating || busy}
          aria-label="New project name"
        />
        <button
          type="submit"
          className="gt-btn gt-btn--secondary"
          disabled={isCreating || busy}
        >
          {isCreating ? "Creating…" : "New project"}
        </button>
        {createError ? (
          <p className="gt-sidebar__error" role="alert">
            {createError}
          </p>
        ) : null}
      </form>

      {error ? (
        <p className="gt-sidebar__error" role="alert">
          {error}
        </p>
      ) : null}

      {actionError ? (
        <p className="gt-sidebar__error" role="alert">
          {actionError}
        </p>
      ) : null}

      {isLoading ? <p className="gt-muted">Loading projects…</p> : null}

      {!isLoading && projects.length === 0 && archivedCount === 0 ? (
        <p className="gt-muted">No projects yet. Create one above.</p>
      ) : null}

      {allArchived ? (
        <p className="gt-muted">
          All projects are archived. Restore one below or create a new project.
        </p>
      ) : null}

      <ul className="gt-project-list">
        {projects.map((project) => {
          const isActive = project.id === selectedId;
          return (
            <li key={project.id} className="gt-project-item">
              <button
                type="button"
                className={isActive ? "gt-project gt-project--active" : "gt-project"}
                aria-current={isActive ? "true" : undefined}
                onClick={() => onSelect(project.id)}
                disabled={busy}
              >
                <span className="gt-project__name">{project.name}</span>
                <span className="gt-project__meta">
                  Last used {formatLastUsed(project.last_used_at)}
                </span>
              </button>
              <ProjectCardMenu
                projectName={project.name}
                isOpen={openMenuId === project.id}
                onOpenChange={(open) => setOpenMenuId(open ? project.id : null)}
                onRename={() => onRenameRequest(project)}
                onRemove={() => void handleArchive(project)}
                disabled={busy}
                removing={archivingId === project.id}
              />
            </li>          );
        })}
      </ul>

      {archivedCount > 0 ? (
        <div className="gt-sidebar__archived">
          <button
            type="button"
            className="gt-sidebar__archived-toggle"
            onClick={onToggleArchived}
            aria-expanded={showArchived}
          >
            {showArchived ? "Hide archived" : `Show archived (${archivedCount})`}
          </button>

          {showArchived ? (
            <ul className="gt-project-list gt-project-list--archived">
              {archivedProjects.map((project) => (
                <li key={project.id} className="gt-project-item gt-project-item--archived">
                  <div className="gt-project gt-project--archived">
                    <span className="gt-project__name">{project.name}</span>
                    <span className="gt-project__meta">
                      Archived {formatArchivedAt(project.archived_at)}
                    </span>
                  </div>
                  <button
                    type="button"
                    className="gt-project__action"
                    onClick={() => void handleRestore(project.id)}
                    disabled={busy || restoringId === project.id}
                    aria-label={`Restore ${project.name}`}
                    title="Restore project"
                  >
                    {restoringId === project.id ? "…" : "Restore"}
                  </button>
                </li>
              ))}
            </ul>
          ) : null}
        </div>
      ) : null}
    </aside>
  );
}
