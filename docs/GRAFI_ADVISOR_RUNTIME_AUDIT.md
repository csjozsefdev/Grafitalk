# Grafi Advisor — Runtime Audit Report

**Date:** 2026-06-15  
**Upstream pin:** [csjozsefdev/Grafi](https://github.com/csjozsefdev/Grafi) @ `23fd1c3dce9bda624decbabd4f84742226f8f5bc`  
**Environment:** Vite dev server (`http://localhost:5173/`) with Tauri API mocks — same React/CSS bundle as Tauri WebView in dev  
**Viewport:** 1280×820 CSS px, `devicePixelRatio` 1.25  
**Collector:** `scripts/grafi-advisor-runtime-audit.mjs`

## Executive summary

Grafi Advisor **renders and mounts correctly** in GrafiTalk. The integration (portal → `.gt-grafi-host` → upstream `GrafiAdvisor` with `placement="static"`) is structurally sound. The asset loads at full resolution and scales to the upstream default figure box (108×150 px).

What looks “broken” is **visual/contextual**, not a failed mount:

1. **Dark upstream bubble tokens** (`--grafi-surface: #141414`) produce a near-black speech card on GrafiTalk’s light workbench — reads as a black overlay.
2. **Host anchor `left: 24px`** places the figure inside the 248 px sidebar column; the bubble panel extends into the main document area.
3. **Persistent boot message** (`welcome`) starts **expanded** because `GrafiTalkAdvisor` always sets `displayMode: "expanded"` for new messages and only auto-collapses *transient* events.

No evidence of crushed layout, missing PNG, nested fixed hosts, or viewport clipping.

### Visual correction (2026-06-15)

Follow-up comparison with **Graf-Id** showed the “shattered texture” is **not** a portal/mount defect:

| | GrafiTalk (before) | Graf-Id |
|---|-------------------|---------|
| Figure asset | upstream `grafi-transparent.png` (550 KB) | `grafi-transparent-v5.png` (81 KB) |
| Workbench | light gray / white | dark |
| Symptom | green/black fringe pixels, figure looks “szétesett” | cohesive robot |

Fix: use v5 display asset (see `PROVENANCE.md`). No synthetic backdrop — v5 alpha is transparent.

---

## Artifacts

| File | Description |
|------|-------------|
| [01-baseline-workbench.png](./qa-screenshots/grafi-advisor-audit/01-baseline-workbench.png) | Workbench with Grafi expanded (boot state) |
| [02-debug-outlines.png](./qa-screenshots/grafi-advisor-audit/02-debug-outlines.png) | Same view with debug outlines on host / figure / panel |
| [runtime-measurements.json](./qa-screenshots/grafi-advisor-audit/runtime-measurements.json) | Full geometry + computed-style dump |
| [failure-diagnostics.json](./qa-screenshots/grafi-advisor-audit/failure-diagnostics.json) | Earlier failed run (pre-Tauri mock) — retained for traceability |

---

## Render tree (confirmed at runtime)

```
document.body
└── div.gt-grafi-host                    [fixed, left:24, bottom:32, z-index:1000]
    └── section.grafi-advisor.grafi-advisor--placement-static.grafi-advisor--expanded
        └── div.grafi-bubble.grafi-bubble--expanded
            ├── div.grafi-bubble__figure-column   [108×150]
            │   └── img.grafi-figure__img         [931×1150 natural, object-fit:contain]
            └── div.grafi-bubble__panel           [288×102, bg rgb(20,20,20)]
```

---

## Measured geometry

| Element | x | y | width | height | Notes |
|---------|---|---|-------|--------|-------|
| `.gt-grafi-host` | 24 | 638 | 406 | 150 | Fixed anchor; height = figure column |
| `.grafi-figure` / image | 24 | 638 | 108 | 150 | Upstream default size from `GrafiFigure.css` |
| `.grafi-bubble__panel` | 142 | 646 | 288 | 102 | Dark surface; `margin-bottom: 2.5rem` (40 px) |
| `.gt-sidebar` | 0 | 76 | 248 | 771 | Figure sits within sidebar horizontal band |
| `.gt-layout` | 0 | 76 | 1280 | 771 | Main grid below title bar |

**Viewport overflow (Grafi subtree):** all `left/top/right/bottomOverflow` = **0** — nothing clipped off-screen.

**Document scroll:** `scrollHeight` 848 vs `innerHeight` 820 (+28 px). Source is app shell layout, not Grafi clipping.

---

## Hit-test stacking (elementsFromPoint)

| Probe point | Top elements (first → last) | Interpretation |
|-------------|----------------------------|----------------|
| Host center | `grafi-bubble__message` → `grafi-bubble__panel` → … → `gt-sidebar` | Panel receives pointer events; sidebar is underneath |
| Figure center | `grafi-figure__img` → … → `gt-sidebar` | Figure paints above sidebar content |
| Panel center | `grafi-bubble__message` → … → `gt-document__email-body` | Panel overlaps main preview column |

Host uses `pointer-events: none`; child advisor restores `pointer-events: auto` — matches design.

---

## Image asset

| Property | Value |
|----------|-------|
| URL | `/src/vendor/grafi/assets/grafi-transparent.png` |
| Natural size | 931 × 1150 |
| Display size | 108 × 150 |
| `object-fit` | `contain` |
| `object-position` | `center bottom` |
| `complete` | `true` |

The figure is **intentionally small** per upstream CSS, not crushed by GrafiTalk overrides (none exist on `.grafi-figure*`).

---

## Hypothesis matrix

| # | Hypothesis | Result | Evidence |
|---|------------|--------|----------|
| A | Dark upstream panel looks like a “black square/card” on light UI | **CONFIRMED** | `panel.computed.backgroundColor = rgb(20, 20, 20)`; `--grafi-surface: #141414` in vendored `grafi.css` |
| B | Persistent messages always start expanded | **CONFIRMED** | Class `grafi-advisor--expanded`; boot message `welcome` has no `transient` flag; adapter line 74 sets `"expanded"` on every new message id |
| C | Host at `left: 24px` overlaps sidebar | **CONFIRMED** | Figure left edge 24 px; sidebar width 248 px; hit-test shows `gt-sidebar` under figure |
| D | Panel intrudes into main workspace | **CONFIRMED** | Panel right edge 430 px; sidebar ends 248 px; panel hit-test reaches `gt-document` |
| E | Figure asset crushed / wrong dimensions | **REJECTED** | 108×150 matches upstream `GrafiFigure.css`; PNG loads at 931×1150 |
| F | Nested fixed positioning / double host | **REJECTED** | Single `.gt-grafi-host`; advisor `position: relative` with `placement-static` |
| G | Viewport clip hides part of Grafi | **REJECTED** | Zero overflow on all Grafi elements |
| H | GrafiTalk CSS overrides break bubble layout | **REJECTED** | Only `GrafiTalkAdvisorHost.css` positions host; no figure/panel overrides |
| I | Missing / broken asset | **REJECTED** | Image complete, correct natural dimensions |
| J | Tauri-only crash prevents render | **N/A in this run** | Pre-mock run failed on `TitleBar`; with `__TAURI_INTERNALS__` mock, layout renders identically to dev WebView bundle |

---

## Upstream comparison (static)

Upstream Grafi @ `23fd1c3` ships the same:

- Row bubble layout (`flex-direction: row; align-items: flex-end`)
- Panel `margin-bottom: 2.5rem` (lifts tail above figure baseline)
- Dark design tokens on `.grafi-advisor`
- Default `displayMode = 'minimized'` in `GrafiAdvisor.tsx` — **GrafiTalk adapter overrides this to `"expanded"` on each new message**

On upstream’s own dark demo page the panel blends in. In GrafiTalk’s light workbench the **same CSS reads as broken** — this is a host-app theme/placement mismatch, not corrupt vendored code.

---

## Root-cause ranking

| Rank | Cause | User-visible symptom | Fix surface |
|------|-------|---------------------|-------------|
| 1 | Upstream dark `--grafi-surface` unmodified | Black/dark speech card on white/gray workbench | Scoped light tokens on `.gt-grafi-host` (adapter CSS only) |
| 2 | Host `left: 24px` ignores 248 px sidebar | Robot + bubble cover sidebar labels / “Projects” area | Adjust host `left` to clear sidebar (e.g. `calc(248px + 1rem)`) or responsive rule |
| 3 | Adapter expands all new messages | Large footprint on every boot / workflow state change | Start persistent messages minimized; keep expand for transient flash events |
| 4 | Upstream row geometry + panel width | Bubble extends ~180 px into main column | Secondary; may be acceptable after 1–3 |

**Do not:** replace/crop the PNG, edit vendored `grafi.css`, or reintroduce legacy overlay CSS.

---

## Recommended minimal corrections (next task)

1. **Light theme scope** — override `--grafi-surface`, `--grafi-text`, borders, and panel shadow inside `.gt-grafi-host` only.
2. **Placement** — move host right of sidebar at desktop widths; keep mobile safe-area fallback.
3. **Display mode policy** — `welcome`, `empty-context`, `draft-ready`, etc.: initial `minimized`; transient events: expand then auto-collapse (existing 5.5 s timer).

---

## Reproduction

```bash
# Terminal 1 — dev server (Tauri or Vite)
npx @tauri-apps/cli dev

# Terminal 2 — audit collector
node scripts/grafi-advisor-runtime-audit.mjs
```

Optional: `GRAFI_AUDIT_URL=http://127.0.0.1:5173/` if the dev URL differs.

---

## Limitations

- Audit used **Playwright + Tauri invoke mock**, not OS-level WebView automation. CSS/DOM results match the Tauri dev bundle; native window chrome and real IPC timing were not measured.
- Single viewport (1280×820). Narrow/mobile layouts not captured in this pass.
- Boot state only (`welcome` message, no project selected). Transient flash events not exercised in screenshots.
