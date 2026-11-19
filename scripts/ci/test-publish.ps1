# Trockenprobe von publish.ps1 ohne Netz: lokales Git-Repository als Gegenstelle, Wegwerf-Schlüssel, Attrappen-Setup.
# Prüft: leeres Repository wird angelegt, Katalog wird signiert und ist gültig, zweite Version kommt dazu,
# dieselbe Version wird abgelehnt, tasks/latest.json liegt da. Die GitHub-Release-Datei (gh) wird hier ausgelassen.
param([Parameter(Mandatory)][string]$CatalogTool, [Parameter(Mandatory)][string]$LauncherCli)
$ErrorActionPreference = "Stop"
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$t = Join-Path ([System.IO.Path]::GetTempPath()) "daedyx-publish-test-$PID"
New-Item -ItemType Directory -Force $t | Out-Null
try {
  # Wegwerf-Schlüssel (nur für diese Probe)
  & $CatalogTool keygen --out (Join-Path $t "keys") --no-password | Out-Null
  $env:DAEDYX_CATALOG_KEY = Get-Content (Join-Path $t "keys\catalog.key") -Raw
  $env:DAEDYX_KEY_PASSWORD = "unbenutzt-in-der-probe"
  $pub = Join-Path $t "keys\catalog.pub"
  if (-not (Test-Path $pub)) { $pub = Get-ChildItem (Join-Path $t "keys") -Filter "*.pub" -Recurse | Select-Object -First 1 -ExpandProperty FullName }

  # leere Gegenstelle
  $remote = Join-Path $t "remote.git"
  git init --quiet --bare --initial-branch=main $remote
  if ($LASTEXITCODE -ne 0) { throw "git init" }

  function New-Out($version) {
    $bundle = Join-Path $t "bundle$version"; $out = Join-Path $t "out$version"
    New-Item -ItemType Directory -Force $bundle | Out-Null
    [IO.File]::WriteAllBytes((Join-Path $bundle "Daedyx Tasks_${version}_x64-setup.exe"), [byte[]](1..200))
    Set-Content (Join-Path $bundle "Daedyx Tasks_${version}_x64-setup.exe.sig") "SIGNATUR" -Encoding ascii
    $notes = Join-Path $t "notes$version.txt"; Set-Content $notes "Probe $version" -Encoding utf8
    node (Join-Path $root "scripts\lib\release-cli.mjs") --dir $bundle --out $out --version $version --notes-file $notes `
      --base-url "https://github.com/daedyx/releases/releases/download/tasks-v$version/"
    if ($LASTEXITCODE -ne 0) { throw "release-cli" }
    return @{ Out = $out; Notes = $notes }
  }

  function Publish($version) {
    $o = New-Out $version
    & (Join-Path $PSScriptRoot "publish.ps1") -Version $version -OutDir $o.Out -NotesFile $o.Notes -PublicKey $pub `
      -CatalogTool $CatalogTool -ReleasesRemote $remote -SkipGhRelease *>&1 | Out-Host
    return [int]$LASTEXITCODE
  }

  if ((Publish "0.4.1") -ne 0) { throw "erste Veröffentlichung fehlgeschlagen" }
  if ((Publish "0.4.2") -ne 0) { throw "zweite Veröffentlichung fehlgeschlagen" }
  $again = $true
  try { if ((Publish "0.4.2") -eq 0) { $again = $false } } catch { }
  if (-not $again) { throw "dieselbe Version wurde nicht abgelehnt" }

  # Ergebnis prüfen: Katalog aus der Gegenstelle mit dem echten Launcher-CLI lesen (prüft Signatur und Schema)
  $check = Join-Path $t "check"
  git clone --quiet $remote $check 2>$null
  foreach ($f in "catalog.json", "catalog.json.minisig", "catalog.pub", "tasks\latest.json", "README.md", ".gitattributes") {
    if (-not (Test-Path (Join-Path $check $f))) { throw "Datei fehlt im Ergebnis: $f" }
  }
  $list = & $LauncherCli --catalog $check --pubkey (Join-Path $check "catalog.pub") --data-dir (Join-Path $t "ldata") list
  Write-Output $list
  if (($list -join " ") -notmatch "neueste: 0\.4\.2") { throw "Launcher sieht nicht 0.4.2" }
  $cat = Get-Content (Join-Path $check "catalog.json") -Raw | ConvertFrom-Json
  $v = $cat.apps[0].releases | ForEach-Object { $_.version }
  if (($v -join ",") -ne "0.4.1,0.4.2") { throw "Versionen im Katalog: $($v -join ',')" }
  if ($cat.apps[0].registryKey -ne "Daedyx Tasks") { throw "registryKey fehlt" }
  if ($cat.apps[0].releases[1].packages.'windows-x64'.url -ne "https://github.com/daedyx/releases/releases/download/tasks-v0.4.2/Daedyx.Tasks_0.4.2_x64-setup.exe") { throw "Paketadresse falsch" }
  Write-Output "Trockenprobe bestanden."
} finally {
  Remove-Item -Recurse -Force $t -ErrorAction SilentlyContinue
}
