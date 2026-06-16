import { useCallback, useEffect, useMemo, useRef, useState } from "react";

import {
  exportDatabaseBackup,
  getAppDiagnostics,
  openDataFolder,
  type AppDiagnostics,
} from "../api/app";
import { getProjectContext, saveProjectContext } from "../api/context";
import {
  exportFormatFromPath,
  formatReviewTemplateLabel,
  getProjectReviewMetadata,
  migrateLastGeneratedAtIfEmpty,
  recordDraftExport,
  type ProjectReviewMetadata,
} from "../api/reviewMetadata";
import {
  generateDraft,
  getProjectDraft,
  saveProjectDraft,
  saveProjectTemplateKind,
} from "../api/draft";
import {
  buildSuggestedExportFilename,
  prepareDraftExport,
} from "../api/export";
import {
  formatHintFromPath,
  importGrafIdHandoff,
} from "../api/import";
import {
  archiveProject,
  createProject,
  listArchivedProjects,
  listProjects,
  MAX_PROJECT_NAME_LEN,
  renameProject,
  restoreProject,
  touchProjectUsed,
} from "../api/projects";
import { useGrafiFlash } from "../hooks/useGrafiFlash";
import { useConfirmDialog } from "../hooks/useConfirmDialog";
import { useNamePromptDialog } from "../hooks/useNamePromptDialog";
import { useReadAloud } from "../hooks/useReadAloud";
import type { Project } from "../types/project";
import {
  loadGrafiTalkPreferences,
  saveGrafiTalkPreferences,
  type GrafiTalkPreferences,
} from "../types/grafiPreferences";
import {
  DEFAULT_TEMPLATE_KIND,
  templateTitle,
  type TemplateKind,
} from "../types/template";
import {
  deriveWorkflowStep,
  type SessionCompletion,
} from "../utils/deriveWorkflowStep";
import { isDraftDirty, normalizeDraftText } from "../utils/draftDirtyState";
import { formatRelativeFromIso } from "../utils/formatRelativeShort";
import { errorMessage } from "../utils/errorMessage";
import {
  ERROR_FEEDBACK_MS,
  SESSION_COMPLETION_MS,
  showFeedback,
} from "../utils/feedback";
import { loadLastGeneratedByProject } from "../utils/lastGeneratedStorage";
import { DraftActionBar, OutputActionBar } from "./ActionBar";
import { ConfirmDialog } from "./ConfirmDialog";
import { ContextPanel } from "./ContextPanel";
import { DocumentPanel } from "./DocumentPanel";
import { GrafiTalkAdvisor } from "./GrafiTalkAdvisor";
import { NamePromptDialog } from "./NamePromptDialog";
import { ProjectSidebar } from "./ProjectSidebar";
import { SettingsView } from "./SettingsView";
import { TitleBar } from "./TitleBar";
import type { SettingsCategoryId } from "../types/settingsCategories";

const SAVE_DEBOUNCE_MS = 500;

export interface AppShellProps {
  onStartupReady?: () => void;
}

export function AppShell({ onStartupReady }: AppShellProps = {}) {
  const [projects, setProjects] = useState<Project[]>([]);
  const [archivedProjects, setArchivedProjects] = useState<Project[]>([]);
  const [showArchived, setShowArchived] = useState(false);
  const [sidebarActionError, setSidebarActionError] = useState<string | null>(null);
  const [selectedId, setSelectedId] = useState("");
  const [isLoading, setIsLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [contextText, setContextText] = useState("");
  const [contextLoading, setContextLoading] = useState(false);
  const [contextSaveError, setContextSaveError] = useState<string | null>(null);
  const [importFeedback, setImportFeedback] = useState<string | null>(null);
  const [isImporting, setIsImporting] = useState(false);
  const [draftText, setDraftText] = useState("");
  const [draftLoading, setDraftLoading] = useState(false);
  const [draftSaveError, setDraftSaveError] = useState<string | null>(null);
  const [templateKind, setTemplateKind] = useState<TemplateKind>(DEFAULT_TEMPLATE_KIND);
  const [isGenerating, setIsGenerating] = useState(false);
  const [copyFeedback, setCopyFeedback] = useState<string | null>(null);
  const [generateFeedback, setGenerateFeedback] = useState<string | null>(null);
  const [exportFeedback, setExportFeedback] = useState<string | null>(null);
  const [isExporting, setIsExporting] = useState(false);
  const [reviewMetadata, setReviewMetadata] = useState<ProjectReviewMetadata | null>(
    null
  );
  const [sessionCompletion, setSessionCompletion] =
    useState<SessionCompletion>("none");
  const [preferences, setPreferences] = useState<GrafiTalkPreferences>(() =>
    loadGrafiTalkPreferences()
  );
  const [workspaceView, setWorkspaceView] = useState<"workbench" | "settings">(
    "workbench"
  );
  const [settingsCategory, setSettingsCategory] =
    useState<SettingsCategoryId>("general");
  const [diagnostics, setDiagnostics] = useState<AppDiagnostics | null>(null);
  const [diagnosticsLoading, setDiagnosticsLoading] = useState(false);
  const [diagnosticsBusy, setDiagnosticsBusy] = useState(false);
  const [diagnosticsFeedback, setDiagnosticsFeedback] = useState<string | null>(
    null
  );

  const selectedIdRef = useRef(selectedId);
  const loadGenerationRef = useRef(0);
  const contextSaveTimerRef = useRef<number | null>(null);
  const draftSaveTimerRef = useRef<number | null>(null);
  const pendingContextSaveRef = useRef<{ projectId: string; text: string } | null>(
    null
  );
  const pendingDraftSaveRef = useRef<{ projectId: string; text: string } | null>(
    null
  );
  const [generatedDraftBaseline, setGeneratedDraftBaseline] = useState<
    string | null
  >(null);
  const startupReadyCalledRef = useRef(false);

  const readAloud = useReadAloud(preferences.readAloudEnabled);
  const { confirm: requestConfirm, dialogProps: confirmDialogProps } =
    useConfirmDialog();
  const { prompt: requestNamePrompt, dialogProps: namePromptDialogProps } =
    useNamePromptDialog();

  useEffect(() => {
    selectedIdRef.current = selectedId;
  }, [selectedId]);

  useEffect(() => {
    if (sessionCompletion === "none") {
      return;
    }

    const timer = window.setTimeout(() => {
      setSessionCompletion("none");
    }, SESSION_COMPLETION_MS);

    return () => window.clearTimeout(timer);
  }, [sessionCompletion]);

  useEffect(() => {
    readAloud.stop();
  }, [selectedId, draftText, readAloud]);

  const clearSaveTimers = useCallback(() => {
    if (contextSaveTimerRef.current !== null) {
      window.clearTimeout(contextSaveTimerRef.current);
      contextSaveTimerRef.current = null;
    }
    if (draftSaveTimerRef.current !== null) {
      window.clearTimeout(draftSaveTimerRef.current);
      draftSaveTimerRef.current = null;
    }
  }, []);

  const refreshArchivedProjects = useCallback(async () => {
    try {
      const loaded = await listArchivedProjects();
      setArchivedProjects(loaded);
    } catch (err) {
      console.error("Failed to load archived projects", err);
    }
  }, []);

  const refreshProjects = useCallback(async (preferSelectId?: string) => {
    const loaded = await listProjects();
    setProjects(loaded);
    await refreshArchivedProjects();

    if (loaded.length === 0) {
      setSelectedId("");
      return;
    }

    if (preferSelectId && loaded.some((project) => project.id === preferSelectId)) {
      setSelectedId(preferSelectId);
      return;
    }

    setSelectedId((current) =>
      loaded.some((project) => project.id === current) ? current : loaded[0].id
    );
  }, [refreshArchivedProjects]);

  const persistContext = useCallback(async (projectId: string, text: string) => {
    try {
      await saveProjectContext(projectId, text);
      setContextSaveError(null);
    } catch (err) {
      console.error("Failed to save context", err);
      setContextSaveError(
        err instanceof Error ? err.message : "Failed to save context"
      );
    }
  }, []);

  const persistDraft = useCallback(async (projectId: string, text: string) => {
    try {
      await saveProjectDraft(projectId, text);
      setDraftSaveError(null);
    } catch (err) {
      console.error("Failed to save draft", err);
      setDraftSaveError(
        err instanceof Error ? err.message : "Failed to save draft"
      );
    }
  }, []);

  const flushPendingSaves = useCallback(async () => {
    clearSaveTimers();

    const contextPending = pendingContextSaveRef.current;
    const draftPending = pendingDraftSaveRef.current;
    pendingContextSaveRef.current = null;
    pendingDraftSaveRef.current = null;

    const saves: Promise<unknown>[] = [];
    if (contextPending) {
      saves.push(persistContext(contextPending.projectId, contextPending.text));
    }
    if (draftPending) {
      saves.push(persistDraft(draftPending.projectId, draftPending.text));
    }
    if (saves.length > 0) {
      await Promise.all(saves);
    }
  }, [clearSaveTimers, persistContext, persistDraft]);

  useEffect(() => {
    let cancelled = false;

    async function load() {
      setIsLoading(true);
      setLoadError(null);
      try {
        await refreshProjects();
      } catch (err) {
        console.error("Failed to load projects", err);
        if (!cancelled) {
          setLoadError(err instanceof Error ? err.message : "Failed to load projects");
        }
      } finally {
        if (!cancelled) {
          setIsLoading(false);
        }
      }
    }

    void load();
    return () => {
      cancelled = true;
    };
  }, [refreshProjects]);

  useEffect(() => {
    if (isLoading || startupReadyCalledRef.current) {
      return;
    }

    startupReadyCalledRef.current = true;
    onStartupReady?.();
  }, [isLoading, onStartupReady]);

  const loadReviewMetadata = useCallback(async (projectId: string) => {
    let metadata = await getProjectReviewMetadata(projectId);

    if (!metadata.lastGeneratedAt) {
      const legacyMs = loadLastGeneratedByProject()[projectId];
      if (legacyMs) {
        const migrated = await migrateLastGeneratedAtIfEmpty(
          projectId,
          new Date(legacyMs).toISOString()
        );
        if (migrated) {
          metadata = await getProjectReviewMetadata(projectId);
        }
      }
    }

    return metadata;
  }, []);

  useEffect(() => {
    if (!selectedId) {
      return;
    }

    const generation = ++loadGenerationRef.current;

    async function loadProjectData() {
      setContextLoading(true);
      setDraftLoading(true);
      setContextSaveError(null);
      setDraftSaveError(null);
      setSessionCompletion("none");

      try {
        const [context, draft, metadata] = await Promise.all([
          getProjectContext(selectedId),
          getProjectDraft(selectedId),
          loadReviewMetadata(selectedId),
        ]);
        if (generation !== loadGenerationRef.current) {
          return;
        }
        setContextText(context);
        setDraftText(draft);
        setGeneratedDraftBaseline(normalizeDraftText(draft));
        setTemplateKind(metadata.lastTemplateKind);
        setReviewMetadata(metadata);
      } catch (err) {
        console.error("Failed to load project data", err);
        if (generation !== loadGenerationRef.current) {
          return;
        }
        const message =
          err instanceof Error ? err.message : "Failed to load project data";
        setContextSaveError(message);
        setDraftSaveError(message);
        setContextText("");
        setDraftText("");
        setGeneratedDraftBaseline(null);
      } finally {
        if (generation === loadGenerationRef.current) {
          setContextLoading(false);
          setDraftLoading(false);
        }
      }
    }

    void loadProjectData();
  }, [loadReviewMetadata, selectedId]);

  useEffect(() => {
    return () => {
      void flushPendingSaves();
    };
  }, [flushPendingSaves]);

  const selectedProject = useMemo(
    () => projects.find((project) => project.id === selectedId) ?? null,
    [projects, selectedId]
  );

  const documentTitle = selectedProject
    ? `${templateTitle(templateKind)} — ${selectedProject.name}`
    : "Select a project";

  const hasContext = selectedId ? contextText.trim().length > 0 : false;
  const draftReady = selectedId ? draftText.trim().length > 0 : false;
  const draftIsDirty = isDraftDirty(draftText, generatedDraftBaseline);

  const activeWorkflowStep = deriveWorkflowStep({
    selectedId: selectedId || null,
    hasContext,
    draftReady,
    isGenerating,
    sessionCompletion,
  });

  const { message: grafiMessage, emitGrafiEvent } = useGrafiFlash({
    preferences,
    selectedId: selectedId || null,
    hasContext,
    draftReady,
    isDraftDirty: draftIsDirty,
    sessionCompletion,
    activeProjectCount: projects.length,
    allProjectsArchived:
      !isLoading && projects.length === 0 && archivedProjects.length > 0,
  });

  const regenerateDraftForTemplate = useCallback(
    async (projectId: string, kind: TemplateKind) => {
      setIsGenerating(true);
      setGenerateFeedback(null);

      try {
        await flushPendingSaves();
        const draft = await generateDraft(projectId, kind);

        if (selectedIdRef.current !== projectId) {
          return;
        }

        setDraftText(draft);
        setGeneratedDraftBaseline(normalizeDraftText(draft));
        pendingDraftSaveRef.current = null;
        setDraftSaveError(null);

        const metadata = await getProjectReviewMetadata(projectId);
        if (selectedIdRef.current === projectId) {
          setReviewMetadata(metadata);
        }
      } catch (err) {
        console.error("Failed to regenerate draft for template", err);
        if (selectedIdRef.current === projectId) {
          showFeedback(
            setGenerateFeedback,
            err instanceof Error ? err.message : "Failed to regenerate draft",
            ERROR_FEEDBACK_MS
          );
        }
      } finally {
        setIsGenerating(false);
      }
    },
    [flushPendingSaves]
  );

  const handleTemplateChange = useCallback(
    async (kind: TemplateKind) => {
      if (!selectedId) {
        setTemplateKind(kind);
        return;
      }

      const previousKind = templateKind;
      if (
        kind !== previousKind &&
        isDraftDirty(draftText, generatedDraftBaseline)
      ) {
        emitGrafiEvent("templateOverwriteRisk");
        const confirmed = await requestConfirm({
          title: "Replace edited draft?",
          description:
            "Changing the template will replace your manual edits to this draft. Continue?",
          confirmLabel: "Change template",
          tone: "default",
        });
        if (!confirmed) {
          return;
        }
      }

      setTemplateKind(kind);

      try {
        await saveProjectTemplateKind(selectedId, kind);
      } catch (err) {
        console.error("Failed to save template kind", err);
      }

      if (!contextText.trim()) {
        return;
      }

      await regenerateDraftForTemplate(selectedId, kind);
    },
    [
      contextText,
      draftText,
      emitGrafiEvent,
      generatedDraftBaseline,
      regenerateDraftForTemplate,
      requestConfirm,
      selectedId,
      templateKind,
    ]
  );

  const handleContextChange = useCallback(
    (value: string) => {
      setContextText(value);
      setSessionCompletion("none");
      if (!selectedId) return;

      pendingContextSaveRef.current = { projectId: selectedId, text: value };

      if (contextSaveTimerRef.current !== null) {
        window.clearTimeout(contextSaveTimerRef.current);
      }

      contextSaveTimerRef.current = window.setTimeout(() => {
        const pending = pendingContextSaveRef.current;
        if (!pending) return;
        pendingContextSaveRef.current = null;
        void persistContext(pending.projectId, pending.text);
      }, SAVE_DEBOUNCE_MS);
    },
    [persistContext, selectedId]
  );

  const handleBodyChange = useCallback(
    (value: string) => {
      setDraftText(value);
      setSessionCompletion("none");
      if (!selectedId) return;

      pendingDraftSaveRef.current = { projectId: selectedId, text: value };

      if (draftSaveTimerRef.current !== null) {
        window.clearTimeout(draftSaveTimerRef.current);
      }

      draftSaveTimerRef.current = window.setTimeout(() => {
        const pending = pendingDraftSaveRef.current;
        if (!pending) return;
        pendingDraftSaveRef.current = null;
        void persistDraft(pending.projectId, pending.text);
      }, SAVE_DEBOUNCE_MS);
    },
    [persistDraft, selectedId]
  );

  const handleSelectProject = useCallback(
    async (id: string) => {
      if (id === selectedId) return;
      await flushPendingSaves();
      readAloud.stop();
      setSessionCompletion("none");
      setSelectedId(id);
      try {
        await touchProjectUsed(id);
        await refreshProjects(id);
      } catch (err) {
        console.error("Failed to update project last used", err);
      }
    },
    [flushPendingSaves, readAloud, refreshProjects, selectedId]
  );

  const handleGenerate = useCallback(async () => {
    if (!selectedId) {
      showFeedback(setGenerateFeedback, "Select a project first", ERROR_FEEDBACK_MS);
      return;
    }

    if (!contextText.trim()) {
      showFeedback(
        setGenerateFeedback,
        "Add context before generating a draft.",
        ERROR_FEEDBACK_MS
      );
      return;
    }

    const projectId = selectedId;
    const activeTemplate = templateKind;

    setIsGenerating(true);
    setGenerateFeedback(null);

    try {
      await flushPendingSaves();

      const draft = await generateDraft(projectId, activeTemplate);

      if (selectedIdRef.current !== projectId) {
        console.warn("Discarded stale generate result", { projectId });
        return;
      }

      setDraftText(draft);
      setGeneratedDraftBaseline(normalizeDraftText(draft));
      pendingDraftSaveRef.current = null;
      setDraftSaveError(null);

      const metadata = await getProjectReviewMetadata(projectId);
      if (selectedIdRef.current === projectId) {
        setReviewMetadata(metadata);
        setTemplateKind(metadata.lastTemplateKind);
      }
      showFeedback(setGenerateFeedback, "Draft generated");
    } catch (err) {
      console.error("Failed to generate draft", err);
      if (selectedIdRef.current === projectId) {
        showFeedback(
          setGenerateFeedback,
          err instanceof Error ? err.message : "Failed to generate draft",
          ERROR_FEEDBACK_MS
        );
      }
    } finally {
      setIsGenerating(false);
    }
  }, [contextText, flushPendingSaves, selectedId, templateKind]);

  const handleCopy = useCallback(async () => {
    if (!draftText.trim()) {
      showFeedback(setCopyFeedback, "Nothing to copy yet", ERROR_FEEDBACK_MS);
      return;
    }

    try {
      await navigator.clipboard.writeText(draftText);
      setSessionCompletion("copied");
      emitGrafiEvent("copySucceeded");
      showFeedback(setCopyFeedback, "Copied to clipboard");
    } catch (err) {
      console.error("Failed to copy draft", err);
      showFeedback(setCopyFeedback, "Copy failed — select text manually", ERROR_FEEDBACK_MS);
    }
  }, [draftText, emitGrafiEvent]);

  const handleExport = useCallback(async () => {
    if (!selectedId || !selectedProject) {
      showFeedback(setExportFeedback, "Select a project first", ERROR_FEEDBACK_MS);
      return;
    }

    if (!draftText.trim()) {
      showFeedback(setExportFeedback, "Nothing to export yet", ERROR_FEEDBACK_MS);
      return;
    }

    const projectId = selectedId;
    const activeTemplate = templateKind;
    const draftSnapshot = draftText;

    setIsExporting(true);
    setExportFeedback(null);

    try {
      await flushPendingSaves();

      const { save } = await import("@tauri-apps/plugin-dialog");
      const defaultPath = buildSuggestedExportFilename(
        selectedProject.name,
        activeTemplate,
        "txt"
      );

      const targetPath = await save({
        defaultPath,
        filters: [
          { name: "Plain text", extensions: ["txt"] },
          { name: "Markdown", extensions: ["md"] },
          { name: "JSON", extensions: ["json"] },
          { name: "PDF", extensions: ["pdf"] },
        ],
      });

      if (targetPath === null) {
        return;
      }

      const prepared = await prepareDraftExport(
        projectId,
        activeTemplate,
        draftSnapshot,
        { pathHint: targetPath }
      );

      if (selectedIdRef.current !== projectId) {
        console.warn("Discarded stale export result", { projectId });
        return;
      }

      if (prepared.text !== null) {
        const { writeTextFile } = await import("@tauri-apps/plugin-fs");
        await writeTextFile(targetPath, prepared.text);
      } else if (prepared.bytes !== null) {
        const { writeFile } = await import("@tauri-apps/plugin-fs");
        await writeFile(targetPath, new Uint8Array(prepared.bytes));
      } else {
        throw new Error("Export did not return any content.");
      }

      await recordDraftExport(projectId, exportFormatFromPath(targetPath));
      const metadata = await getProjectReviewMetadata(projectId);
      if (selectedIdRef.current === projectId) {
        setReviewMetadata(metadata);
      }

      setSessionCompletion("exported");
      emitGrafiEvent("exportSucceeded");
      showFeedback(setExportFeedback, "Draft exported");
    } catch (err) {
      console.error("Failed to export draft", err);
      emitGrafiEvent("exportFailed");
      if (selectedIdRef.current === projectId) {
        showFeedback(
          setExportFeedback,
          err instanceof Error ? err.message : "Failed to export draft",
          ERROR_FEEDBACK_MS
        );
      }
    } finally {
      setIsExporting(false);
    }
  }, [
    draftText,
    emitGrafiEvent,
    flushPendingSaves,
    selectedId,
    selectedProject,
    templateKind,
  ]);

  const handleImport = useCallback(async () => {
    if (!selectedId) {
      showFeedback(setImportFeedback, "Select a project first", ERROR_FEEDBACK_MS);
      return;
    }

    const projectId = selectedId;

    setIsImporting(true);
    setImportFeedback(null);

    try {
      await flushPendingSaves();

      const { open } = await import("@tauri-apps/plugin-dialog");
      const { readTextFile } = await import("@tauri-apps/plugin-fs");

      const selected = await open({
        multiple: false,
        directory: false,
        filters: [
          {
            name: "Graf-ID handoff",
            extensions: ["json", "txt", "md", "markdown"],
          },
        ],
      });

      if (selected === null) {
        return;
      }

      const selectedPath = Array.isArray(selected) ? selected[0] : selected;
      if (typeof selectedPath !== "string" || !selectedPath.trim()) {
        throw new Error("Import dialog returned an invalid file path.");
      }

      const content = await readTextFile(selectedPath);
      const formatHint = formatHintFromPath(selectedPath);

      const context = await importGrafIdHandoff(projectId, content, {
        formatHint,
        pathHint: selectedPath,
      });

      if (selectedIdRef.current !== projectId) {
        console.warn("Discarded stale import result", { projectId });
        return;
      }

      setContextText(context);
      pendingContextSaveRef.current = null;
      setContextSaveError(null);
      setSessionCompletion("none");
      emitGrafiEvent("importSucceeded");
      showFeedback(setImportFeedback, "Context imported");
    } catch (err) {
      console.error("Failed to import context", err);
      if (selectedIdRef.current === projectId) {
        showFeedback(
          setImportFeedback,
          errorMessage(err, "Failed to import context"),
          ERROR_FEEDBACK_MS
        );
      }
    } finally {
      setIsImporting(false);
    }
  }, [emitGrafiEvent, flushPendingSaves, selectedId]);

  const handleCreate = useCallback(
    async (name: string) => {
      await flushPendingSaves();
      const created = await createProject(name);
      setSelectedId(created.id);
      try {
        await touchProjectUsed(created.id);
      } catch (err) {
        console.error("Failed to touch newly created project", err);
      }
      await refreshProjects(created.id);
    },
    [flushPendingSaves, refreshProjects]
  );

  const handleArchive = useCallback(
    async (projectId: string) => {
      setSidebarActionError(null);
      await flushPendingSaves();
      readAloud.stop();
      try {
        await archiveProject(projectId);
      } catch (err) {
        setSidebarActionError(
          err instanceof Error ? err.message : "Failed to archive project"
        );
        throw err;
      }

      const wasSelected = projectId === selectedId;
      await refreshProjects();

      if (wasSelected) {
        setContextText("");
        setDraftText("");
        setGeneratedDraftBaseline(null);
        setReviewMetadata(null);
        setSessionCompletion("none");
      }
    },
    [flushPendingSaves, readAloud, refreshProjects, selectedId]
  );

  const handleArchiveRequest = useCallback(
    async (project: Project) => {
      const confirmed = await requestConfirm({
        title: `Remove "${project.name}"?`,
        description:
          "Remove this project from the active list? This will not delete exported files.",
        confirmLabel: "Remove",
        tone: "destructive",
      });

      if (!confirmed) {
        return;
      }

      try {
        await handleArchive(project.id);
      } catch {
        // Error surfaced via sidebarActionError.
      }
    },
    [handleArchive, requestConfirm]
  );

  const handleRenameRequest = useCallback(
    async (project: Project) => {
      const nextName = await requestNamePrompt({
        title: `Rename "${project.name}"`,
        initialValue: project.name,
        confirmLabel: "Rename",
        maxLength: MAX_PROJECT_NAME_LEN,
      });

      if (!nextName || nextName === project.name) {
        return;
      }

      setSidebarActionError(null);
      await flushPendingSaves();

      try {
        await renameProject(project.id, nextName);
        await refreshProjects(project.id);
      } catch (err) {
        setSidebarActionError(
          err instanceof Error ? err.message : "Failed to rename project"
        );
      }
    },
    [flushPendingSaves, refreshProjects, requestNamePrompt]
  );

  const handleRestore = useCallback(
    async (projectId: string) => {
      setSidebarActionError(null);
      try {
        await restoreProject(projectId);
        await refreshProjects();
      } catch (err) {
        setSidebarActionError(
          err instanceof Error ? err.message : "Failed to restore project"
        );
      }
    },
    [refreshProjects]
  );

  const openSettings = useCallback((category: SettingsCategoryId = "general") => {
    setSettingsCategory(category);
    setWorkspaceView("settings");
  }, []);

  const closeSettings = useCallback(() => {
    setWorkspaceView("workbench");
  }, []);

  const loadDiagnostics = useCallback(async () => {
    setDiagnosticsFeedback(null);
    setDiagnosticsLoading(true);
    try {
      const loaded = await getAppDiagnostics();
      setDiagnostics(loaded);
    } catch (err) {
      setDiagnosticsFeedback(
        err instanceof Error ? err.message : "Failed to load diagnostics"
      );
    } finally {
      setDiagnosticsLoading(false);
    }
  }, []);

  const openDiagnostics = useCallback(async () => {
    openSettings("about");
    await loadDiagnostics();
  }, [loadDiagnostics, openSettings]);

  const handlePreferencesChange = useCallback((next: GrafiTalkPreferences) => {
    setPreferences(next);
    saveGrafiTalkPreferences(next);
  }, []);

  const handleBackup = useCallback(async () => {
    setDiagnosticsBusy(true);
    setDiagnosticsFeedback(null);
    try {
      const { save } = await import("@tauri-apps/plugin-dialog");
      const targetPath = await save({
        defaultPath: "grafitalk-backup.db",
        filters: [{ name: "SQLite database", extensions: ["db"] }],
      });
      if (targetPath === null) {
        return;
      }
      await exportDatabaseBackup(targetPath);
      setDiagnosticsFeedback("Database backup saved.");
    } catch (err) {
      setDiagnosticsFeedback(
        err instanceof Error ? err.message : "Failed to export database backup"
      );
    } finally {
      setDiagnosticsBusy(false);
    }
  }, []);

  const handleOpenDataFolder = useCallback(async () => {
    setDiagnosticsBusy(true);
    setDiagnosticsFeedback(null);
    try {
      const folder = await openDataFolder();
      setDiagnosticsFeedback(`Data folder: ${folder}`);
    } catch (err) {
      setDiagnosticsFeedback(
        err instanceof Error ? err.message : "Failed to resolve data folder"
      );
    } finally {
      setDiagnosticsBusy(false);
    }
  }, []);

  const panelBusy = !selectedId || contextLoading || draftLoading || isImporting || isExporting;
  const panelContextText = selectedId ? contextText : "";
  const panelDraftText = selectedId ? draftText : "";
  const contextSaved = hasContext && !contextSaveError && !contextLoading;
  const activeReviewMetadata = selectedId ? reviewMetadata : null;
  const lastGeneratedLabel = formatRelativeFromIso(
    activeReviewMetadata?.lastGeneratedAt ?? null
  );
  const templateLabel = activeReviewMetadata
    ? formatReviewTemplateLabel(activeReviewMetadata.lastTemplateKind)
    : templateTitle(templateKind);
  const lastExportedLabel = activeReviewMetadata?.lastExportedAt
    ? `${formatRelativeFromIso(activeReviewMetadata.lastExportedAt)} (${activeReviewMetadata.lastExportFormat?.toUpperCase() ?? "FILE"})`
    : null;

  return (
    <div className="gt-app">
      <div className="gt-ambient" aria-hidden="true" />
      <TitleBar
        activeStep={activeWorkflowStep}
        onOpenDiagnostics={() => {
          void openDiagnostics();
        }}
      />
      <div
        className={
          workspaceView === "settings"
            ? "gt-layout gt-layout--settings"
            : "gt-layout"
        }
      >
        <ProjectSidebar
          projects={projects}
          archivedProjects={archivedProjects}
          showArchived={showArchived}
          onToggleArchived={() => setShowArchived((value) => !value)}
          selectedId={selectedId}
          onSelect={(id) => {
            void handleSelectProject(id);
          }}
          onCreate={handleCreate}
          onArchiveRequest={handleArchiveRequest}
          onRenameRequest={(project) => {
            void handleRenameRequest(project);
          }}
          onRestore={handleRestore}
          onOpenSettings={() => openSettings("general")}
          settingsActive={workspaceView === "settings"}
          isLoading={isLoading}
          error={loadError}
          actionError={sidebarActionError}
          busy={panelBusy || isGenerating}
        />
        {workspaceView === "settings" ? (
          <SettingsView
            key={settingsCategory}
            preferences={preferences}
            onPreferencesChange={handlePreferencesChange}
            onBack={closeSettings}
            initialCategory={settingsCategory}
            diagnostics={diagnostics}
            diagnosticsLoading={diagnosticsLoading}
            diagnosticsFeedback={diagnosticsFeedback}
            diagnosticsBusy={diagnosticsBusy}
            onLoadDiagnostics={() => {
              void loadDiagnostics();
            }}
            onBackup={() => {
              void handleBackup();
            }}
            onOpenDataFolder={() => {
              void handleOpenDataFolder();
            }}
          />
        ) : (
          <>
            <main className="gt-main">
              <DocumentPanel
                title={documentTitle}
                body={panelDraftText}
                onBodyChange={handleBodyChange}
                disabled={panelBusy || isGenerating || isExporting}
                saveError={draftSaveError}
                draftReady={draftReady}
                contextSaved={contextSaved}
                lastGeneratedLabel={lastGeneratedLabel}
                templateLabel={templateLabel}
                lastExportedLabel={lastExportedLabel}
                readAloudSupported={readAloud.supported}
                readAloudEnabled={preferences.readAloudEnabled}
                readAloudReadiness={readAloud.readiness}
                readAloudError={readAloud.lastError}
                isSpeaking={readAloud.isSpeaking}
                onReadAloud={() => readAloud.toggle(panelDraftText)}
                outputActions={
                  <OutputActionBar
                    onCopy={handleCopy}
                    onExport={() => {
                      void handleExport();
                    }}
                    copyFeedback={copyFeedback}
                    exportFeedback={exportFeedback}
                    copyDisabled={panelBusy || !draftReady}
                    exportDisabled={panelBusy || !draftReady}
                    isExporting={isExporting}
                  />
                }
                draftActions={
                  <DraftActionBar
                    templateKind={templateKind}
                    onTemplateChange={handleTemplateChange}
                    onGenerate={handleGenerate}
                    generateFeedback={generateFeedback}
                    generateDisabled={panelBusy || !hasContext}
                    templateDisabled={panelBusy}
                    isGenerating={isGenerating}
                    isExporting={isExporting}
                  />
                }
              />
            </main>
            <ContextPanel
              value={panelContextText}
              onChange={handleContextChange}
              onImport={() => {
                void handleImport();
              }}
              disabled={panelBusy}
              importDisabled={!selectedId}
              importFeedback={importFeedback}
              saveError={contextSaveError}
            />
          </>
        )}
      </div>
      <GrafiTalkAdvisor message={grafiMessage} preferences={preferences} />
      <ConfirmDialog {...confirmDialogProps} />
      <NamePromptDialog {...namePromptDialogProps} />
    </div>
  );
}
