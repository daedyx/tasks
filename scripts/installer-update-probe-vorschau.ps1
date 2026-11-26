# Installationsprobe des Updates aus der laufenden App (U2, plan-update-bibliothek 10, Muster M8). Nur Kennungen der
# Vorschau. Ablauf: Setup -From still in einen Wegwerf-Ordner installieren, einmal starten und beenden (Datenbank mit
# erfundenen Aufgaben entsteht), dann die laufende App das Setup -To selbst starten lassen (DAEDYX_UPDATE_PROBE, nur in
# der Vorschau übersetzt): App beendet sich, Programm ersetzt, Neustart, Ordner gleich, app.json = -To, Daten gleich.
# Dasselbe versteckt (--hidden): Die neu gestartete App bleibt versteckt. Zum Schluss Deinstallation (/S), alles weg.
# Den Nachweis braucht die Probe nicht (geprüft in den Tests von daedyx-update und daedyx-updater).
# Rückgabe 0 = alles in Ordnung, 1 = Fehler, 2 = Abbruch vor jedem Schreiben. Gibt nur OK/FEHLER und Zahlen aus.
# Beispiel: powershell -ExecutionPolicy Bypass -File scripts\installer-update-probe-vorschau.ps1 `
#   -From 0.0.1 -Setup1 installer\out\daedyx-tasks-vorschau-0.0.1-setup.exe `
#   -To 0.0.2 -Setup2 installer\out\daedyx-tasks-vorschau-0.0.2-setup.exe
param(
    [Parameter(Mandatory = $true)][string]$From,
    [Parameter(Mandatory = $true)][string]$Setup1,
    [Parameter(Mandatory = $true)][string]$To,
    [Parameter(Mandatory = $true)][string]$Setup2,
    [string]$Dir = (Join-Path $env:TEMP "daedyx-tasks-vorschau-update-probe-$PID")
)
$product = "Daedyx Tasks Vorschau"
$exe = "daedyx-tasks-vorschau.exe"
$uninstKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\$product"
$dirKey = "HKCU:\Software\daedyx\$product"
$aumidKey = "HKCU:\Software\Classes\AppUserModelId\de.daedyx.tasks.vorschau"
$runKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"
$lnk = Join-Path ([Environment]::GetFolderPath("Programs")) "$product.lnk"
$dataDir = Join-Path $env:APPDATA "de.daedyx.tasks.vorschau"
# Lokaler Ordner der Vorschau: geladene Setups (Review U2-R N-U2-4).
$localDir = Join-Path $env:LOCALAPPDATA "de.daedyx.tasks.vorschau"
$realData = Join-Path $env:APPDATA "de.daedyx.tasks"
$realUninst = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\Daedyx Tasks"
$realDirKey = "HKCU:\Software\daedyx\Daedyx Tasks"
$script:failed = 0

function Check($name, $ok) { if ($ok) { "OK      $name" } else { "FEHLER  $name"; $script:failed++ } }
function Run($file, $argLine, $ms = 120000) {
    $p = Start-Process $file -ArgumentList $argLine -PassThru
    if ($p.WaitForExit($ms)) { return $p.ExitCode }
    Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
    return "haengt"
}
function Procs { @(Get-Process "daedyx-tasks-vorschau" -ErrorAction SilentlyContinue | Where-Object { $_.Path -like "$Dir\*" }) }
function Wait-Until($ms, [scriptblock]$cond) {
    $sw = [Diagnostics.Stopwatch]::StartNew()
    while ($sw.ElapsedMilliseconds -lt $ms) { if (& $cond) { return $true }; Start-Sleep -Milliseconds 250 }
    return [bool](& $cond)
}
function AppVersion { if (Test-Path "$Dir\app.json") { (Get-Content "$Dir\app.json" -Raw | ConvertFrom-Json).version } }
function RealState {
    $d = (Get-ItemProperty $realDirKey -ErrorAction SilentlyContinue).'(default)'
    $r = (Get-ItemProperty $runKey -Name "DaedyxTasks" -ErrorAction SilentlyContinue).DaedyxTasks
    "u=$(Test-Path $realUninst)|d=$d|r=$r|data=$(Test-Path -LiteralPath $realData)"
}
# Nur Namen und Prüfsummen der Datenbank-Dateien, nie Inhalte.
function DataState {
    if (-not (Test-Path $dataDir)) { return "fehlt" }
    (Get-ChildItem $dataDir -Recurse -File | Sort-Object FullName | ForEach-Object {
        $_.FullName.Substring($dataDir.Length) + "=" + (Get-FileHash $_.FullName -Algorithm MD5).Hash }) -join ";"
}
# Sichtbarkeit des App-Fensters (Klasse „Window Class“ von winit mit Titel). MainWindowHandle taugt nicht: Es trifft das
# unsichtbare Hilfsfenster „Winit Thread Event Target“, das immer als sichtbar gilt.
Add-Type -Namespace DaedyxProbe -Name Win -MemberDefinition @"
public delegate bool CB(System.IntPtr h, System.IntPtr l);
[System.Runtime.InteropServices.DllImport("user32.dll")] public static extern bool EnumWindows(CB cb, System.IntPtr l);
[System.Runtime.InteropServices.DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(System.IntPtr h, out uint pid);
[System.Runtime.InteropServices.DllImport("user32.dll")] public static extern bool IsWindowVisible(System.IntPtr h);
[System.Runtime.InteropServices.DllImport("user32.dll", CharSet=System.Runtime.InteropServices.CharSet.Unicode)] public static extern int GetWindowTextLength(System.IntPtr h);
[System.Runtime.InteropServices.DllImport("user32.dll", CharSet=System.Runtime.InteropServices.CharSet.Unicode)] public static extern int GetClassName(System.IntPtr h, System.Text.StringBuilder s, int n);
public static int AppWindow(uint want) { int r = -1; EnumWindows((h, l) => { uint p; GetWindowThreadProcessId(h, out p); var c = new System.Text.StringBuilder(64); GetClassName(h, c, 64); if (p == want && c.ToString() == "Window Class" && GetWindowTextLength(h) > 0) { r = IsWindowVisible(h) ? 1 : 0; } return true; }, System.IntPtr.Zero); return r; }
"@
function Quit-App {
    $null = Run "$Dir\$exe" "--quit-running" 20000
    Wait-Until 20000 { (Procs).Count -eq 0 }
}

# Schutz (vor jedem Schreiben): echte App nicht installiert und ohne Datenordner, Vorschau nirgends vorhanden.
$realDir = (Get-ItemProperty $realDirKey -ErrorAction SilentlyContinue).'(default)'
if ((Test-Path $realUninst) -or ($realDir -and (Test-Path -LiteralPath $realDir)) -or (Test-Path -LiteralPath $realData)) {
    'Abbruch: Die echte App "Daedyx Tasks" ist installiert oder hat einen Datenordner (nichts verändert).'; exit 2
}
if ((Test-Path $uninstKey) -or (Test-Path $dirKey) -or (Test-Path $aumidKey) -or (Test-Path $lnk) -or
    ($null -ne (Get-ItemProperty $runKey -Name "DaedyxTasksVorschau" -ErrorAction SilentlyContinue)) -or (Test-Path $dataDir) -or
    (Test-Path $localDir)) {
    'Abbruch: Die Vorschau ist schon installiert oder hat Daten. Die Probe fasst sie nicht an (nichts verändert).'; exit 2
}
if (-not (Test-Path $Setup1) -or -not (Test-Path $Setup2) -or $From -eq $To) { "Abbruch: Setups fehlen oder gleiche Version."; exit 2 }
$realBefore = RealState
# Probe-Umgebung ohne Setup: Die App sucht dann nicht im Netz (probe.rs, `active`), startet aber auch nichts.
$env:DAEDYX_UPDATE_PROBE = "aus"; $env:DAEDYX_UPDATE_PROBE_VON = $null

Check "0 still installieren $From (/S /D=)" ((Run $Setup1 "/S /D=$Dir") -eq 0)
Check "0 app.json $From" ((AppVersion) -eq $From)
$exeHash1 = (Get-FileHash "$Dir\$exe").Hash
$first = Start-Process "$Dir\$exe" -PassThru
Start-Sleep -Seconds 4
Check "0 erster Start legt die Datenbank an" (Test-Path (Join-Path $dataDir "daedyx-tasks.db"))
Check "0 beenden (--quit-running)" (Quit-App)
$data = DataState

foreach ($mode in @("sichtbar", "versteckt")) {
    $tag = if ($mode -eq "sichtbar") { "A" } else { "B" }
    if ($tag -eq "B") {
        Check "B erneut $From installieren" ((Run $Setup1 "/S /D=$Dir") -eq 0 -and (AppVersion) -eq $From)
    }
    $env:DAEDYX_UPDATE_PROBE = (Resolve-Path $Setup2).Path
    $env:DAEDYX_UPDATE_PROBE_VON = $From
    $argList = if ($tag -eq "B") { @("--hidden") } else { @() }
    $old = if ($argList.Count) { Start-Process "$Dir\$exe" -ArgumentList $argList -PassThru } else { Start-Process "$Dir\$exe" -PassThru }
    Check "$tag $mode gestartete App startet das Setup und beendet sich selbst" ($old.WaitForExit(60000))
    Check "$tag Setup ersetzt das Programm (app.json $To)" (Wait-Until 60000 { (AppVersion) -eq $To })
    Check "$tag App neu gestartet, im selben Ordner" (Wait-Until 30000 { (Procs).Count -eq 1 })
    $u = Get-ItemProperty $uninstKey -ErrorAction SilentlyContinue
    Check "$tag Uninstall-Eintrag: Version $To, Ordner gleich" ($u -and $u.DisplayVersion -eq $To -and $u.InstallLocation -eq $Dir)
    Check "$tag Programmdatei ersetzt (andere Prüfsumme)" ((Get-FileHash "$Dir\$exe").Hash -ne $exeHash1)
    Start-Sleep -Seconds 3
    $p = Procs | Select-Object -First 1
    if ($tag -eq "A") { Check "A neu gestartete App zeigt ihr Fenster" ($p -and [DaedyxProbe.Win]::AppWindow([uint32]$p.Id) -eq 1) }
    else { Check "B neu gestartete App bleibt versteckt" ($p -and [DaedyxProbe.Win]::AppWindow([uint32]$p.Id) -eq 0 -and -not $p.HasExited) }
    Check "$tag kein zweites Update in der neu gestarteten App" ((AppVersion) -eq $To -and (Procs).Count -eq 1)
    Check "$tag beenden (--quit-running)" (Quit-App)
    Check "$tag Daten unverändert (Dateien und Prüfsummen)" ((DataState) -eq $data)
    $env:DAEDYX_UPDATE_PROBE = "aus"; $env:DAEDYX_UPDATE_PROBE_VON = $null
}
$env:DAEDYX_UPDATE_PROBE = $null

Check "C Deinstallation (/S)" ((Run "$Dir\uninstall.exe" "/S") -eq 0)
Start-Sleep -Seconds 3
Check "C Ordner entfernt" (-not (Test-Path $Dir))
Check "C Uninstall-Eintrag, Merkschlüssel, App-Kennung, Verknüpfung entfernt" (-not ((Test-Path $uninstKey) -or (Test-Path $dirKey) -or (Test-Path $aumidKey) -or (Test-Path $lnk)))
Check "C Datenordner der Vorschau entfernt (--cleanup)" (-not (Test-Path $dataDir))
Check "C lokaler Ordner der Vorschau (Setups) fehlt" (-not (Test-Path $localDir))
Check "C Tresor-Eintrag der Vorschau entfernt (nur Name geprüft)" (-not ((cmdkey /list) -match "db-key\.daedyx-tasks-vorschau"))
Check "C keine Prozesse" ((Procs).Count -eq 0)
Check "C echte App unverändert (nur gelesen)" ((RealState) -eq $realBefore)
Remove-Item $Dir -Recurse -Force -ErrorAction SilentlyContinue
if ($script:failed -eq 0) { "ERGEBNIS: alles in Ordnung"; exit 0 } else { "ERGEBNIS: $($script:failed) Fehler"; exit 1 }
