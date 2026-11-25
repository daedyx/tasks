# Baut das Setup der egui-Vorschau (installer/tasks-vorschau.nsi) lokal: Release-Bau des Programms, dann makensis.
# Den Ablauf liefert die Vorlage aus daedyx-build (Tasks T1b, `installer-bauen.ps1`); hier stehen nur die Werte von
# Tasks (T6). Lädt nichts herunter. Ausgabe: installer/out/daedyx-tasks-vorschau-<Version>-setup.exe, Größe, SHA-256.
# Beispiel: powershell -ExecutionPolicy Bypass -File scripts\installer-vorschau.ps1 -Offline
param(
    [string]$Version = "",
    [string]$Makensis = "",
    [string]$TargetDir = $env:CARGO_TARGET_DIR,
    [switch]$SkipBuild,
    [switch]$Offline
)
$ErrorActionPreference = "Stop"
. (Join-Path $PSScriptRoot "lib\vorlage-installer.ps1")
$repo = Split-Path $PSScriptRoot -Parent
$tauri = Join-Path $repo "app\src-tauri"
$manifest = Join-Path $tauri "crates\tasks-app\Cargo.toml"

if (-not $Version) {
    $line = Select-String -Path $manifest -Pattern '^version\s*=\s*"([^"]+)"' | Select-Object -First 1
    if (-not $line) { throw "Version in tasks-app/Cargo.toml nicht gefunden" }
    $Version = $line.Matches[0].Groups[1].Value
}
$vorlage = Get-InstallerVorlage $tauri -Offline:$Offline
& (Join-Path $vorlage "installer-bauen.ps1") -Workspace $tauri -Package "daedyx-tasks-app" -Exe "daedyx-tasks-vorschau.exe" `
    -Nsi (Join-Path $repo "installer\tasks-vorschau.nsi") -Icon (Join-Path $tauri "icons\icon.ico") -Version $Version `
    -OutDir (Join-Path $repo "installer\out") -Makensis $Makensis -TargetDir $TargetDir -SkipBuild:$SkipBuild -Offline:$Offline
