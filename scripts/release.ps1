Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "Running backend tests..."
cargo test -p backend
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Building frontend..."
npm run build --prefix frontend
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Running frontend unit tests..."
npm run test --prefix frontend
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Checking Tauri app..."
cargo check -p grafitalk
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Building release installer..."
npx @tauri-apps/cli build
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Release artifacts are under target/release/bundle/"
