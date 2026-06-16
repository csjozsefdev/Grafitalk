# Grafi Splash — vendored provenance

| Field | Value |
|-------|-------|
| Source repository | https://github.com/csjozsefdev/Grafi-splash |
| Commit SHA | `345efd03cb6b73b75b40dcd22c2aad2c4c03eb37` |
| License | Not specified upstream (treat as same author ecosystem; confirm on update) |

## Files imported

From `src/components/`:

- `GrafiSplash.tsx`
- `GrafiSplash.css`

**GrafiTalk splash image:** `assets/white-grafi-splash.jpg` — white-background centerpiece with baked-in ring (replaces masked `grafi-splash.png` head crop).

**Imported for reference (upstream default):** `assets/grafi-splash.png` — dark-background upstream asset; not used by GrafiTalk splash.

## Local modifications

### `GrafiSplash.tsx`

- Added optional `imageSrc`, `imageAlt`, and `showLoadingRing` props for GrafiTalk.
- Added ESLint disable for upstream `Date.now` / effect setState patterns preserved from Grafi-splash.
- Default `imageSrc` points to vendored `assets/grafi-splash.png`.

### GrafiTalk white-background variant (`GrafiTalkSplash.css`)

- Near-white full-screen background (`#ffffff` → `#f4f6f8` gradient).
- Centered splash image at compact size (`260px`–`340px` width, height auto).
- No radial mask, dark card, or CSS loading ring (`showLoadingRing={false}`).

## GrafiTalk integration

- Wrapper: `frontend/src/components/grafi-splash/GrafiTalkSplash.tsx`
- White theme overrides: `frontend/src/components/grafi-splash/GrafiTalkSplash.css`

## Update procedure

1. Check upstream `main` for new commits.
2. Re-copy `GrafiSplash.tsx` and `GrafiSplash.css`.
3. Re-apply `imageSrc` / `imageAlt` props patch.
4. Verify `GrafiTalkSplash.css` white-variant overrides still apply cleanly.
5. Update this file with the new commit SHA.
