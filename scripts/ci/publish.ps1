# Veröffentlicht eine gebaute Tasks-Version in das Repository daedyx/releases (läuft in der GitHub-Action):
#  1. Release-Datei (Setup) unter dem Tag tasks-v<Version> anlegen,
#  2. catalog.json um den Eintrag ergänzen, mit dem Katalog-Schlüssel signieren und prüfen,
#  3. tasks/latest.json (für den Updater der App) ablegen, committen, pushen.
# Reihenfolge: erst die Datei, dann der Katalog, damit der Katalog nie auf eine fehlende Datei zeigt.
# Geheimnisse nur über Umgebungsvariablen: RELEASES_TOKEN (Schreibrecht auf daedyx/releases),
# DAEDYX_CATALOG_KEY (Inhalt der geheimen Schlüsseldatei), DAEDYX_KEY_PASSWORD. Nichts davon wird ausgegeben.
param(
  [Parameter(Mandatory)][string]$Version,
  [Parameter(Mandatory)][string]$OutDir,        # Ergebnis von release.ps1 -BaseUrl: Setup, latest.json, catalog-entry.json
  [Parameter(Mandatory)][string]$NotesFile,
  [Parameter(Mandatory)][string]$PublicKey,     # catalog.pub (öffentlich, liegt im Repository)
  [Parameter(Mandatory)][string]$CatalogTool,   # daedyx-catalog.exe
  [string]$ReleasesRepo = "daedyx/releases",
  [string]$ReleasesRemote,                      # nur für Proben: anderer Git-Remote (zum Beispiel ein lokaler Ordner)
  [switch]$SkipGhRelease                        # nur für Proben: keine Release-Datei bei GitHub anlegen
)
$ErrorActionPreference = "Stop"
$here = Split-Path -Parent $PSScriptRoot
$tag = "tasks-v$Version"
$work = Join-Path ([System.IO.Path]::GetTempPath()) "daedyx-releases-$PID"

function Fail($text) { Write-Error $text; exit 1 }
# Nur der Exit-Code zählt: Windows PowerShell 5.1 macht aus Warnungen auf stderr sonst einen Abbruch
function Invoke-GitCode {
  $old = $ErrorActionPreference; $ErrorActionPreference = "Continue"
  & git.exe @args 2>&1 | ForEach-Object { "$_" } | Write-Host
  $code = $LASTEXITCODE; $ErrorActionPreference = $old
  return $code
}
function Invoke-Git { if ((Invoke-GitCode @args) -ne 0) { throw "git $($args[0]) ist fehlgeschlagen" } }

foreach ($name in "DAEDYX_CATALOG_KEY", "DAEDYX_KEY_PASSWORD") {
  if (-not [Environment]::GetEnvironmentVariable($name)) { Fail "Das Secret $name fehlt." }
}
if (-not $ReleasesRemote -and -not $env:RELEASES_TOKEN) { Fail "Das Secret RELEASES_TOKEN fehlt." }
$setup = Get-ChildItem $OutDir -Filter "*_${Version}_x64-setup.exe" | Select-Object -First 1
if (-not $setup) { Fail "Kein Setup für Version $Version in $OutDir." }
foreach ($f in "latest.json", "catalog-entry.json") {
  if (-not (Test-Path (Join-Path $OutDir $f))) { Fail "$f fehlt in $OutDir." }
}
if (-not (Test-Path $PublicKey)) { Fail "Der öffentliche Katalog-Schlüssel fehlt: $PublicKey" }
# Nach Push-Location gelten relative Pfade nicht mehr: alle Pfad-Parameter jetzt absolut machen.
$PublicKey = (Resolve-Path $PublicKey).Path
$OutDir = (Resolve-Path $OutDir).Path
if ($NotesFile) { $NotesFile = (Resolve-Path $NotesFile).Path }
if ($CatalogTool) { $CatalogTool = (Resolve-Path $CatalogTool).Path }

# Anmeldung für git nur über einen Kopfzeilen-Parameter, das Token steht nie in der Adresse oder in .git/config
$remote = $ReleasesRemote
$gitAuth = @()
if (-not $remote) {
  $remote = "https://github.com/$ReleasesRepo.git"
  $basic = [Convert]::ToBase64String([Text.Encoding]::ASCII.GetBytes("x-access-token:$($env:RELEASES_TOKEN)"))
  $gitAuth = @("-c", "http.extraheader=AUTHORIZATION: basic $basic")
}
function Clone-Releases {
  if (Test-Path $work) { Remove-Item -Recurse -Force $work }
  Invoke-Git @gitAuth clone --quiet $remote $work
  Push-Location $work
  Invoke-Git config user.name "daedyx release bot"
  Invoke-Git config user.email "release-bot@users.noreply.github.com"
  if ((Invoke-GitCode rev-parse --verify HEAD) -ne 0) {
    # leeres Repository: Hauptzweig anlegen, damit Tag und Katalog ein Ziel haben
    Invoke-Git checkout --quiet -b main
    Set-Content (Join-Path $work "README.md") "# Daedyx Releases`n`nSetup-Dateien (Release-Dateien) und der signierte Katalog (catalog.json) der Daedyx-Apps." -Encoding utf8
    Invoke-Git add README.md
    Invoke-Git commit --quiet -m "chore: Repository anlegen"
    Invoke-Git @gitAuth push --quiet origin main
  }
  Pop-Location
}

# 1. Prüfen: Version ist neu, Tag gibt es noch nicht
Clone-Releases
$catalogPath = Join-Path $work "catalog.json"
$existing = if (Test-Path $catalogPath) { $catalogPath } else { "-" }
node (Join-Path $here "lib\catalog-version.mjs") $existing tasks $Version
if ($LASTEXITCODE -ne 0) { Fail "Die Version ist nicht neu." }

# 2. Release-Datei hochladen
if (-not $SkipGhRelease) {
  $env:GH_TOKEN = $env:RELEASES_TOKEN
  & gh release view $tag --repo $ReleasesRepo *> $null
  if ($LASTEXITCODE -eq 0) { Fail "Das Release $tag gibt es schon." }
  & gh release create $tag $setup.FullName --repo $ReleasesRepo --target main --title "Daedyx Tasks $Version" --notes-file $NotesFile
  if ($LASTEXITCODE -ne 0) { Fail "Das Release $tag konnte nicht angelegt werden." }
}

# 3. Katalog ergänzen, signieren, prüfen, committen (bei einem gleichzeitigen Push höchstens dreimal neu)
$pushed = $false
for ($try = 1; $try -le 3 -and -not $pushed; $try++) {
  if ($try -gt 1) { Clone-Releases }
  Push-Location $work
  Copy-Item $PublicKey (Join-Path $work "catalog.pub") -Force
  & $CatalogTool add-release --catalog catalog.json --app tasks --entry (Join-Path $OutDir "catalog-entry.json") `
    --name "Daedyx Tasks" --registry-key "Daedyx Tasks" --description "Aufgaben mit Listen, Abschnitten, Board, Wiederholung und Erinnerung" --author "Daedyx"
  if ($LASTEXITCODE -ne 0) { Pop-Location; Fail "add-release ist fehlgeschlagen." }
  & $CatalogTool sign --catalog catalog.json --key-env DAEDYX_CATALOG_KEY --public catalog.pub
  if ($LASTEXITCODE -ne 0) { Pop-Location; Fail "Das Signieren ist fehlgeschlagen." }
  & $CatalogTool verify --catalog catalog.json --public catalog.pub
  if ($LASTEXITCODE -ne 0) { Pop-Location; Fail "Die Gegenprobe der Signatur ist fehlgeschlagen." }
  New-Item -ItemType Directory -Force (Join-Path $work "tasks") | Out-Null
  Copy-Item (Join-Path $OutDir "latest.json") (Join-Path $work "tasks\latest.json") -Force
  # Signierte Dateien dürfen nie umgewandelt werden (Zeilenenden), sonst passt die Signatur nicht mehr
  # Der Katalog gehört allen Apps: vorhandene Zeilen (z. B. desk/latest.json) bleiben, unsere kommen dazu
  $attrFile = Join-Path $work ".gitattributes"
  $attrs = @(); if (Test-Path $attrFile) { $attrs = @(Get-Content $attrFile | Where-Object { $_.Trim() }) }
  foreach ($line in "catalog.json -text", "catalog.json.minisig -text", "catalog.pub -text", "tasks/latest.json -text") {
    if ($attrs -notcontains $line) { $attrs += $line }
  }
  Set-Content $attrFile ($attrs -join "`n") -Encoding ascii
  Invoke-Git add .gitattributes catalog.json catalog.json.minisig catalog.pub tasks/latest.json
  Invoke-Git commit --quiet -m "release: Daedyx Tasks $Version"
  if ((Invoke-GitCode @gitAuth push --quiet origin main) -eq 0) { $pushed = $true } else { Write-Output "Push abgelehnt (Versuch $try), neu aufsetzen."; }
  Pop-Location
}
if (-not $pushed) { Fail "Der Katalog konnte nicht gepusht werden. Die Release-Datei $tag ist schon da; den Katalog-Eintrag von Hand nachtragen." }

Remove-Item -Recurse -Force $work -ErrorAction SilentlyContinue
Write-Output "Veröffentlicht: Daedyx Tasks $Version ($tag), Katalog und tasks/latest.json aktualisiert."
exit 0
