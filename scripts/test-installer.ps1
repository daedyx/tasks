# Installationsprobe: Setup still in einen Temp-Ordner installieren, app.json und Startprogramm prüfen,
# wieder deinstallieren. Rührt weder den Datenordner noch den Tresor an.
# Eine echte Installation (gleicher Produktname) wird vorher gesichert und danach wiederhergestellt: Registry-Einträge
# (Uninstall und Installationsort) und die Verknüpfungen in Startmenü und auf dem Taskstop. Eine laufende App wird nie
# beendet: das Skript bricht ab, wenn sie läuft.
param(
  [Parameter(Mandatory)][string]$Setup,
  [Parameter(Mandatory)][string]$Version,
  [string]$ProductName = "Daedyx Tasks"
)
$ErrorActionPreference = "Stop"
$dir = Join-Path $env:TEMP "daedyx-tasks-probe-$PID"
$backup = Join-Path $env:TEMP "daedyx-tasks-probe-backup-$PID"

$regKeys = @(
  "HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\$ProductName",
  "HKCU\Software\daedyx\$ProductName"
)
$lnks = @(
  (Join-Path $env:APPDATA "Microsoft\Windows\Start Menu\Programs\$ProductName.lnk"),
  (Join-Path ([Environment]::GetFolderPath("Taskstop")) "$ProductName.lnk")
)

# reg.exe über cmd aufrufen: Windows PowerShell 5.1 macht aus einer Fehlerausgabe sonst einen Abbruch
function Invoke-Reg($arguments) { cmd.exe /c "reg.exe $arguments >nul 2>&1"; return $LASTEXITCODE }
function Test-RegKey($key) { return ((Invoke-Reg "query `"$key`"") -eq 0) }

function Save-Installation {
  New-Item -ItemType Directory -Force $backup | Out-Null
  $saved = @()
  foreach ($key in $regKeys) {
    if (Test-RegKey $key) {
      $file = Join-Path $backup ("reg" + $saved.Count + ".reg")
      if ((Invoke-Reg "export `"$key`" `"$file`" /y") -ne 0) { throw "Registry-Eintrag $key konnte nicht gesichert werden" }
      $saved += [pscustomobject]@{ Key = $key; File = $file }
    }
  }
  $files = @()
  foreach ($l in $lnks) {
    if (Test-Path $l) {
      $copy = Join-Path $backup ("lnk" + $files.Count + ".lnk")
      Copy-Item $l $copy
      $files += [pscustomobject]@{ Path = $l; Copy = $copy }
    }
  }
  return [pscustomobject]@{ Reg = $saved; Lnk = $files }
}

function Restore-Installation($saved) {
  foreach ($key in $regKeys) {
    if (Test-RegKey $key) { Invoke-Reg "delete `"$key`" /f" | Out-Null }
  }
  foreach ($r in $saved.Reg) { Invoke-Reg "import `"$($r.File)`"" | Out-Null }
  foreach ($l in $lnks) { Remove-Item $l -ErrorAction SilentlyContinue }
  foreach ($f in $saved.Lnk) { Copy-Item $f.Copy $f.Path -Force }
}

if (Get-Process -Name "daedyx-tasks" -ErrorAction SilentlyContinue) {
  throw "Daedyx Tasks läuft. Bitte beenden: Die Installationsprobe würde die laufende App schließen."
}
$saved = Save-Installation
try {
  $run = Start-Process -FilePath $Setup -ArgumentList "/S", "/D=$dir" -Wait -PassThru
  if ($run.ExitCode -ne 0) { throw "Setup endete mit Code $($run.ExitCode)" }
  $appJson = Join-Path $dir "app.json"
  if (-not (Test-Path $appJson)) { throw "app.json fehlt nach der Installation" }
  $app = Get-Content $appJson -Raw | ConvertFrom-Json
  if ($app.version -ne $Version) { throw "app.json version $($app.version) statt $Version" }
  if (-not (Test-Path (Join-Path $dir $app.exe))) { throw "Startprogramm $($app.exe) fehlt" }
  $un = Start-Process -FilePath (Join-Path $dir "uninstall.exe") -ArgumentList "/S", "_?=$dir" -Wait -PassThru
  if ($un.ExitCode -ne 0) { throw "Deinstallation endete mit Code $($un.ExitCode)" }
  if (Test-Path $appJson) { throw "app.json nach der Deinstallation noch da" }
  Write-Output "Installationsprobe bestanden ($Version, $($app.exe))"
} finally {
  Restore-Installation $saved
  Remove-Item -Recurse -Force $dir, $backup -ErrorAction SilentlyContinue
}
