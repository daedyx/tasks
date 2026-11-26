# Schreibt entry.json für ein Release (plan-ohne-secrets 2.1 Schritt 4; Schema 1 aus daedyx-update `entry.rs`).
# Liest nur Setup und Notizdatei und schreibt nur -Out; kein Netz, kein Secret. Läuft mit Windows PowerShell 5.1 und
# pwsh. Der Test `release_tests.rs` (tasks-app) prüft die Ausgabe mit dem Prüfweg des Kerns.
# Beispiel: make-entry.ps1 -App daedyx-tasks -Repo daedyx/tasks -Version 0.3.0 -Date 2026-12-24 `
#   -NotesFile notiz.txt -Setup release-dateien\daedyx-tasks-0.3.0-setup.exe -Out release-dateien\entry.json
param(
    [Parameter(Mandatory = $true)][string]$App,
    [Parameter(Mandatory = $true)][string]$Repo,
    [Parameter(Mandatory = $true)][string]$Version,
    [Parameter(Mandatory = $true)][string]$Date,
    [Parameter(Mandatory = $true)][string]$NotesFile,
    [Parameter(Mandatory = $true)][string]$Setup,
    [Parameter(Mandatory = $true)][string]$Out
)
$ErrorActionPreference = "Stop"

# Dieselben Grenzen wie der Prüfweg: lieber hier rot als ein Release, das jede App ablehnt.
if ($App -cnotmatch '^[a-z0-9][a-z0-9-]{0,39}$') { throw "App-Kennung ungültig: $App" }
if ($Repo -notmatch '^[A-Za-z0-9_.-]{1,100}/[A-Za-z0-9_.-]{1,100}$') { throw "Repo ungültig: $Repo" }
if ($Version -notmatch '^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$') { throw "Version muss x.y.z sein: $Version" }
if ($Date -notmatch '^\d{4}-\d{2}-\d{2}$') { throw "Datum muss JJJJ-MM-TT sein: $Date" }
$name = [IO.Path]::GetFileName($Setup)
if ($name -notmatch '^[A-Za-z0-9][A-Za-z0-9._-]{0,195}\.exe$' -or $name.Contains('..')) { throw "Setup-Name ungültig: $name" }

$utf8 = New-Object System.Text.UTF8Encoding $false
$notes = [IO.File]::ReadAllText((Resolve-Path -LiteralPath $NotesFile), $utf8).Replace("`r`n", "`n").Trim()
if ($notes.Length -gt 8000) { throw "Notiz länger als 8000 Zeichen" }
if ($notes -match '[\x00-\x08\x0B-\x1F\x7F]') { throw "Notiz enthält Steuerzeichen" }

$item = Get-Item -LiteralPath $Setup
if ($item.Length -le 0 -or $item.Length -gt 512MB) { throw "Setup-Größe außerhalb der Grenzen: $($item.Length)" }
$sha = (Get-FileHash -LiteralPath $Setup -Algorithm SHA256).Hash.ToLowerInvariant()

$package = [ordered]@{
    url    = "https://github.com/$Repo/releases/download/v$Version/$name"
    sha256 = $sha
    size   = $item.Length
    kind   = "nsis-silent"
}
$entry = [ordered]@{
    schema   = 1
    app      = $App
    version  = $Version
    date     = $Date
    notes    = $notes
    packages = [ordered]@{ "windows-x64" = $package }
}
[IO.File]::WriteAllText([IO.Path]::GetFullPath($Out), ($entry | ConvertTo-Json -Depth 5), $utf8)
"entry.json: $Out"
"Setup:      $name ($($item.Length) Bytes)"
"SHA-256:    $sha"
