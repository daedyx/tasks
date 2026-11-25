# Installationsprobe für das Setup der egui-Vorschau von Tasks (T6, 28 Prüfungen der Vorlage aus daedyx-build, wie Desk
# M8): still installieren, Startprobe, Update /P /UPDATE /R /ARGS, Deinstallation wie Launcher und normal, Autostart-
# Wert weg. Hier stehen nur die Kennungen der Vorschau. Rückgabe 0 = alles in Ordnung, 2 = Abbruch vor jedem Schreiben.
# Schutz der echten App „Daedyx Tasks“ (nur lesen): Ist sie installiert (Uninstall-Eintrag, oder der Ordner-
# Merkschlüssel zeigt auf einen vorhandenen Ordner), bricht die Probe ab. Ihr Autostart-Wert und Merkschlüssel werden
# vorher und nachher verglichen und nie verändert.
# Beispiel: powershell -ExecutionPolicy Bypass -File scripts\installer-probe-vorschau.ps1 -Setup installer\out\daedyx-tasks-vorschau-0.2.0-setup.exe -Offline
param(
    [Parameter(Mandatory = $true)][string]$Setup,
    [string]$Dir = (Join-Path $env:TEMP "daedyx-tasks-vorschau-probe-$PID"),
    [switch]$Offline
)
$realUninst = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\Daedyx Tasks"
$realDirKey = "HKCU:\Software\daedyx\Daedyx Tasks"
$runKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"

# Nur lesen: Zustand der echten App als Fingerabdruck (Werte, keine Ausgabe).
function Get-RealState {
    $dir = (Get-ItemProperty $realDirKey -ErrorAction SilentlyContinue).'(default)'
    $run = (Get-ItemProperty $runKey -Name "DaedyxTasks" -ErrorAction SilentlyContinue).DaedyxTasks
    return "u=$(Test-Path $realUninst)|d=$dir|r=$run"
}
function Test-RealInstalled {
    if (Test-Path $realUninst) { return $true }
    $dir = (Get-ItemProperty $realDirKey -ErrorAction SilentlyContinue).'(default)'
    return [bool]($dir -and (Test-Path -LiteralPath $dir))
}
# Datenordner der echten App (Review T6, N6-1): Auch wenn sie von Hand gelöscht wurde, können dort echte Aufgaben
# liegen. Dann läuft die Probe nicht; sonst wird am Ende verglichen, dass er weiter fehlt (nur lesen, nie öffnen).
$realData = Join-Path $env:APPDATA "de.daedyx.tasks"
function Get-RealDataState {
    if (Test-Path -LiteralPath $realData) { "da|" + (Get-Item -LiteralPath $realData).LastWriteTimeUtc.Ticks } else { "fehlt" }
}
if (Test-Path -LiteralPath $realData) {
    'Abbruch: Der Datenordner der echten App "Daedyx Tasks" existiert. Die Probe läuft nur auf einem Rechner ohne ihn (nichts verändert).'
    exit 2
}
$realDataBefore = Get-RealDataState
if (Test-RealInstalled) {
    'Abbruch: Die echte App "Daedyx Tasks" ist installiert. Die Probe läuft nur auf einem Rechner ohne sie (nichts verändert).'
    exit 2
}
# Datenordner der Vorschau: Die Deinstallation ruft `--cleanup`, das ihn ab T5 samt Tresor-Eintrag entfernt. Liegt
# er schon vorher da, gehört er einer früheren Vorschau; die Probe bricht dann ab, statt fremde Daten zu löschen.
$dataDir = Join-Path $env:APPDATA "de.daedyx.tasks.vorschau"
if (Test-Path -LiteralPath $dataDir) {
    'Abbruch: Der Datenordner der Vorschau existiert schon. Die Probe löscht keine Daten einer früheren Vorschau (nichts verändert).'
    exit 2
}
$before = Get-RealState

. (Join-Path $PSScriptRoot "lib\vorlage-installer.ps1")
$tauri = Join-Path (Split-Path $PSScriptRoot -Parent) "app\src-tauri"
$vorlage = Get-InstallerVorlage $tauri -Offline:$Offline
& (Join-Path $vorlage "installer-probe.ps1") -Setup $Setup -Dir $Dir -Product "Daedyx Tasks Vorschau" `
    -Exe "daedyx-tasks-vorschau.exe" -Aumid "de.daedyx.tasks.vorschau" -RunValue "DaedyxTasksVorschau"
$code = $LASTEXITCODE

# Greift erst mit T5 (vorher legt die Vorschau keinen Ordner an): nach beiden Deinstallationen kein Datenordner.
if (-not (Test-Path -LiteralPath $dataDir)) { 'OK      kein Datenordner der Vorschau nach der Deinstallation' }
else { 'FEHLER  Datenordner der Vorschau nach der Deinstallation noch da'; if ($code -eq 0) { $code = 1 } }
if ((Get-RealState) -eq $before) { 'OK      echte App "Daedyx Tasks" unverändert (nur gelesen)' }
else { 'FEHLER  echte App "Daedyx Tasks" verändert'; if ($code -eq 0) { $code = 1 } }
if ((Get-RealDataState) -eq $realDataBefore) { 'OK      Datenordner der echten App unverändert (nur gelesen)' }
else { 'FEHLER  Datenordner der echten App verändert'; if ($code -eq 0) { $code = 1 } }
exit $code
