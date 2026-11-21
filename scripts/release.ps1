# Baut Daedyx Tasks als Setup und trägt es in den lokalen Katalog des Daedyx Launchers ein.
#
# Zwei Wege (kombinierbar):
#  - Lokaler Katalog (Standard): Setup nach <CatalogDir>\packages, catalog.json eintragen und mit dem Katalog-Schlüssel
#    signieren. Der Launcher installiert und aktualisiert Tasks daraus. Braucht keinen Update-Schlüssel.
#  - Veröffentlichung (-BaseUrl): zusätzlich Setup mit Update-Signatur, latest.json und catalog-entry.json für eine
#    Adresse im Netz. Braucht den Update-Schlüssel (TAURI_SIGNING_PRIVATE_KEY_PATH) und einen echten öffentlichen
#    Schlüssel in tauri.conf.json.
# Das Skript veröffentlicht nichts im Netz und liest keinen privaten Schlüssel selbst (es reicht nur Pfade weiter).
param(
  [Parameter(Mandatory)][string]$Version,
  [Parameter(Mandatory)][string]$Notes,
  [string]$CatalogDir = (Join-Path $env:LOCALAPPDATA "Daedyx\catalog"),
  [string]$CatalogKey = (Join-Path $env:LOCALAPPDATA "Daedyx\keys\catalog.key"),
  [string]$BaseUrl,
  [string]$OutDir,
  [string]$TauriConfig,        # nur für Proben: zusätzliche Tauri-Konfiguration (--config)
  [switch]$NoCatalog,          # den lokalen Katalog nicht anfassen
  [switch]$SkipTests,
  [switch]$NoVersionWrite,     # nur für Proben: Versionsdateien nicht ändern
  [switch]$AllowPlaceholders,  # nur für Proben: Platzhalter-Schlüssel und -Endpunkt zulassen
  [switch]$AllowLocalHttp      # nur für Proben: http://127.0.0.1 als Adresse zulassen
)
$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
$app = Join-Path $root "app"
$conf = Join-Path $app "src-tauri\tauri.conf.json"
if (-not $OutDir) { $OutDir = Join-Path $root "dist-release\$Version" }

function Fail($text) { Write-Error $text; exit 1 }

if ($Version -notmatch '^\d+\.\d+\.\d+$') { Fail "Version muss x.y.z sein: $Version" }
$publish = [bool]$BaseUrl
if ($NoCatalog -and -not $publish) { Fail "Nichts zu tun: weder -BaseUrl noch lokaler Katalog." }
if (-not $NoCatalog -and -not (Test-Path $CatalogKey)) {
  Fail "Der Katalog-Schlüssel fehlt ($CatalogKey). Einmalig erzeugen: node scripts\catalog-key.mjs"
}

if ($publish) {
  # tauri build liest den Update-Schlüssel aus TAURI_SIGNING_PRIVATE_KEY (Pfad oder Inhalt); hier nur der Pfad.
  if ($env:TAURI_SIGNING_PRIVATE_KEY_PATH -and -not $env:TAURI_SIGNING_PRIVATE_KEY) {
    $env:TAURI_SIGNING_PRIVATE_KEY = $env:TAURI_SIGNING_PRIVATE_KEY_PATH
  }
  if (-not $env:TAURI_SIGNING_PRIVATE_KEY) {
    Fail "Der Update-Schlüssel fehlt: TAURI_SIGNING_PRIVATE_KEY_PATH auf die Schlüsseldatei setzen (Björn macht das selbst)."
  }
  $confText = Get-Content $conf -Raw
  if (-not $AllowPlaceholders) {
    if ($confText -match 'PLATZHALTER') { Fail "tauri.conf.json enthält noch den Platzhalter für den öffentlichen Update-Schlüssel." }
    if ($confText -match 'updates\.invalid') { Fail "tauri.conf.json enthält noch den Platzhalter-Endpunkt (updates.invalid)." }
  }
}

if (Get-Process -Name "daedyx-tasks" -ErrorAction SilentlyContinue) {
  Fail "Daedyx Tasks läuft. Bitte beenden: Die Installationsprobe würde die laufende App schließen (Entwürfe gingen verloren)."
}

$env:Path = "C:\Strawberry\perl\bin;C:\Strawberry\c\bin;" + $env:Path + ";$env:USERPROFILE\.cargo\bin"

if (-not $SkipTests) {
  Push-Location $app
  # --retry 2: einzelne zeitabhängige Tests (bekannt, einzeln grün) dürfen unter Last zweimal wiederholt werden
  npm test -- --retry 2; if ($LASTEXITCODE -ne 0) { Pop-Location; Fail "npm test ist rot." }
  # Proben der Freigabe-Regeln und des Installer-Symbols (sonst fiele ein Umbau erst auf, wenn jemand sie von Hand startet)
  npm run test:scripts; if ($LASTEXITCODE -ne 0) { Pop-Location; Fail "npm run test:scripts ist rot." }
  Pop-Location
  powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $root "scripts/ci/test-check-release-inputs.ps1"); if ($LASTEXITCODE -ne 0) { Fail "Probe der Freigabe-Regeln ist rot." }
  Push-Location (Join-Path $app "src-tauri")
  cargo test; if ($LASTEXITCODE -ne 0) { Pop-Location; Fail "cargo test ist rot." }
  Pop-Location
}

# Versionsdateien erst nach grünen Tests ändern, damit ein Abbruch nichts Halbes hinterlässt
if (-not $NoVersionWrite) {
  # UTF-8 ohne BOM lesen und schreiben (Windows PowerShell 5.1 würde sonst Umlaute zerstören und ein BOM anfügen)
  $utf8 = New-Object System.Text.UTF8Encoding $false
  function Set-Version($path, $pattern, $replacement) {
    $text = [System.IO.File]::ReadAllText($path, $utf8)
    [System.IO.File]::WriteAllText($path, [regex]::Replace($text, $pattern, $replacement, 1), $utf8)
  }
  Set-Version $conf '("version":\s*")[^"]+(")' "`${1}$Version`${2}"
  Set-Version (Join-Path $app "src-tauri\Cargo.toml") '(?m)^version = "[^"]+"' "version = `"$Version`""
  Set-Version (Join-Path $app "package.json") '("version":\s*")[^"]+(")' "`${1}$Version`${2}"
}

$buildArgs = @()
$off = $null
if (-not $publish) {
  # Ohne Update-Weg im Netz: keine Update-Signatur nötig. Updates laufen dann nur über den Launcher.
  $off = Join-Path $env:TEMP "daedyx-no-updater-$PID.json"
  Set-Content $off '{"bundle":{"createUpdaterArtifacts":false}}' -Encoding ascii
  $buildArgs += @("--config", $off)
}
if ($TauriConfig) { $buildArgs += @("--config", $TauriConfig) }
Push-Location $app
npm run tauri build -- @buildArgs
$code = $LASTEXITCODE
Pop-Location
if ($off) { Remove-Item $off -ErrorAction SilentlyContinue }
if ($code -ne 0) { Fail "tauri build ist fehlgeschlagen." }

$bundle = Join-Path $app "src-tauri\target\release\bundle\nsis"
$setup = Get-ChildItem $bundle -Filter "*_${Version}_x64-setup.exe" | Select-Object -First 1
if (-not $setup) { Fail "Kein Setup für Version $Version gefunden (Version im Bundle-Ordner prüfen)." }

# Erst probeweise installieren, dann erst in den Katalog schreiben
try {
  & (Join-Path $PSScriptRoot "test-installer.ps1") -Setup $setup.FullName -Version $Version
} catch {
  Fail "Die Installationsprobe ist fehlgeschlagen ($($_.Exception.Message)); nichts in den Katalog geschrieben."
}

$notesFile = Join-Path $env:TEMP "daedyx-notes-$PID.txt"
[System.IO.File]::WriteAllText($notesFile, $Notes, (New-Object System.Text.UTF8Encoding $false))
$nodeArgs = @("--dir", $bundle, "--version", $Version, "--notes-file", $notesFile)
if (-not $NoCatalog) { $nodeArgs += @("--catalog-dir", $CatalogDir, "--catalog-key", $CatalogKey) }
if ($publish) {
  $nodeArgs += @("--base-url", $BaseUrl, "--out", $OutDir)
  if ($AllowLocalHttp) { $nodeArgs += @("--allow-local-http", "true") }
}
node (Join-Path $PSScriptRoot "lib\release-cli.mjs") @nodeArgs
$nodeCode = $LASTEXITCODE
Remove-Item $notesFile -ErrorAction SilentlyContinue
if ($nodeCode -ne 0) { Fail "Katalog-Eintrag konnte nicht erzeugt werden." }

Write-Output ""
Write-Output "Fertig: Daedyx Tasks $Version."
if (-not $NoCatalog) { Write-Output "Lokaler Katalog: $CatalogDir (der Launcher aktualisiert daraus)" }
if ($publish) { Write-Output "Veröffentlichung: $OutDir (Hochladen und Katalog-Signatur im Netz: Björn, siehe docs/RELEASE.md)" }
