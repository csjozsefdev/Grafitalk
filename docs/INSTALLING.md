# Installing GrafiTalk (Windows)

GrafiTalk Core RC 1.0 ships as unsigned Windows installers for trusted testing.

## Download

Build artifacts (after `.\scripts\release.ps1`):

- `target/release/bundle/msi/GrafiTalk_0.1.0_x64_en-US.msi`
- `target/release/bundle/nsis/GrafiTalk_0.1.0_x64-setup.exe`

Verify the file hash published with your release before installing.

## SmartScreen

Windows may show **Windows protected your PC** or similar warnings for unsigned applications. This is expected for RC builds without a code signing certificate.

Only proceed if you trust the release source and have verified the installer hash.

## After install

1. Launch GrafiTalk from the Start menu.
2. Create a project and confirm data persists after restart.
3. See [USER_GUIDE.md](USER_GUIDE.md) for daily use.
4. See [BACKUP_RESTORE.md](BACKUP_RESTORE.md) before replacing your database file.

## Uninstall

Uninstalling via Windows Settings removes the application. Your `grafitalk.db` data file may remain in the app data folder unless you delete it manually.
