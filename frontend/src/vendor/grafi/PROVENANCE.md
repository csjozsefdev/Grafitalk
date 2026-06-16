# Grafi Advisor — vendored provenance

| Field | Value |
|-------|-------|
| Source repository | https://github.com/csjozsefdev/Grafi |
| Commit SHA | `23fd1c3dce9bda624decbabd4f84742226f8f5bc` |
| Package | `grafi-advisor@0.1.0` |
| License | MIT (`LICENSE`) |

## Files imported

From upstream `src/grafi/`:

- `GrafiAdvisor.tsx`, `GrafiBubble.tsx`, `GrafiFigure.tsx`, `GrafiFigure.css`
- `GrafiStatusBadge.tsx`, `GrafiSettingsPanel.tsx`
- `defaultMessages.ts`, `grafiTypes.ts`, `messageVisibility.ts`, `usePrefersReducedMotion.ts`
- `grafi.css`, `grafi-modules.d.ts`, `index.ts`

Assets:

- `assets/grafi-transparent.png` — upstream reference (kept for parity checks)
- `assets/grafi-transparent-v5.png` — **display asset** (from Graf-Id production pipeline; cleaner alpha cutout for advisor UI)

## Local modifications

- `GrafiFigure.tsx` imports `grafi-transparent-v5.png` instead of upstream `grafi-transparent.png`. Upstream PNG is authored for dark canvases; on GrafiTalk’s light workbench its premultiplied glow fringe reads as a “shattered” figure. Graf-Id uses the same v5 asset.

GrafiTalk integrates via:

- `frontend/src/components/GrafiTalkAdvisor.tsx` — host adapter (`createPortal`, `placement="static"`)
- `frontend/src/components/GrafiTalkAdvisorHost.css` — single fixed `.gt-grafi-host` boundary
- `frontend/src/hooks/useGrafiAdvisor.ts`, `useGrafiFlash.ts` — workflow messages and flash events

## GrafiTalk integration

- Mount: `createPortal` → `document.body` → `.gt-grafi-host` → upstream `GrafiAdvisor`
- Upstream default fixed placement is **not** used; host owns viewport position
- Splash remains separate (`vendor/grafi-splash/`)

## Update procedure

1. Fetch upstream `main` and note new commit SHA.
2. Re-copy `src/grafi/*` and `assets/grafi-transparent.png`.
3. Re-apply GrafiTalk adapter/host CSS if API changed.
4. Run `frontend` tests, lint, build.
5. Update this file with the new SHA.
