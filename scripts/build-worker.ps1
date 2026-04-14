# Build the SignalGraph Python worker as a standalone executable
# that Tauri can ship as a sidecar.
#
# Usage (Windows):
#   cd worker
#   ..\scripts\build-worker.ps1
#
# Produces src-tauri\binaries\signalgraph-worker.exe
#
# Requires: python, pip, pyinstaller (pip install pyinstaller).

$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path "$PSScriptRoot\.."
$workerDir = Join-Path $repoRoot "worker"
$outDir = Join-Path $repoRoot "src-tauri\binaries"

if (-not (Test-Path $outDir)) {
    New-Item -ItemType Directory -Path $outDir | Out-Null
}

Push-Location $workerDir
try {
    python -m pip install --quiet pyinstaller

    pyinstaller `
        --onefile `
        --name signalgraph-worker `
        --distpath $outDir `
        --workpath "$workerDir\build" `
        --specpath "$workerDir\build" `
        main.py

    Write-Host "worker bundled to $outDir"
} finally {
    Pop-Location
}
