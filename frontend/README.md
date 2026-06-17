# GrafiTalk frontend

React 19 + TypeScript + Vite UI for the GrafiTalk desktop workbench.

## Commands

```powershell
cd frontend
npm install
npm run dev          # Vite only (IPC calls fail without Tauri)
npm run build
npm run test
npm run lint
```

## Full desktop app

From the repository root:

```powershell
npx @tauri-apps/cli dev
```

See [Developer guide](../docs/DEVELOPER_GUIDE.md) for architecture, IPC, and testing.
