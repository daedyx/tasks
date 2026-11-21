# Probe von check-release-inputs.ps1 ohne Netz: erlaubte und verbotene Kombinationen.
$ErrorActionPreference = 'Stop'
$script = Join-Path $PSScriptRoot 'check-release-inputs.ps1'
$failed = @()

function Expect-Ok($name, $params, $check) {
  try {
    $r = & $script @params
    if (-not (& $check $r)) { $script:failed += "$name : Ergebnis falsch ($($r | ConvertTo-Json -Compress))" }
  } catch { $script:failed += "$name : unerwarteter Fehler: $($_.Exception.Message)" }
}
function Expect-Refused($name, $params, $part) {
  try {
    & $script @params | Out-Null
    $script:failed += "$name : wurde nicht abgelehnt"
  } catch {
    if ($_.Exception.Message -notlike "*$part*") { $script:failed += "$name : falsche Meldung: $($_.Exception.Message)" }
  }
}

$dispatch = @{ EventName = 'workflow_dispatch'; Ref = 'refs/heads/main'; InputVersion = '1.2.3'; InputNotes = 'Neu'; InputDryRun = 'false'; InputSkipTests = 'false'; InputConfirm = '1.2.3' }
function With($base, $over) { $c = $base.Clone(); foreach ($k in $over.Keys) { $c[$k] = $over[$k] }; return $c }

Expect-Ok 'echt von main mit Bestätigung' $dispatch { param($r) $r.Version -eq '1.2.3' -and -not $r.DryRun -and -not $r.SkipTests }
Expect-Refused 'echt von Kandidaten-Zweig' (With $dispatch @{ Ref = 'refs/heads/release-0-7-0-kandidat' }) 'main'
Expect-Refused 'echt von einem Tag-Verweis' (With $dispatch @{ Ref = 'refs/tags/v1.2.3' }) 'main'
Expect-Refused 'echt ohne Bestätigung' (With $dispatch @{ InputConfirm = '' }) 'Bestätigung'
Expect-Refused 'echt mit falscher Bestätigung' (With $dispatch @{ InputConfirm = '1.2.4' }) 'Bestätigung'
Expect-Refused 'echt mit Tests überspringen' (With $dispatch @{ InputSkipTests = 'true' }) 'Trockenlauf'
Expect-Refused 'ungültige Version' (With $dispatch @{ InputVersion = '1.2'; InputConfirm = '1.2' }) 'Version'
Expect-Refused 'Versionstext mit Anhang' (With $dispatch @{ InputVersion = '1.2.3; calc'; InputConfirm = '1.2.3; calc' }) 'Version'
Expect-Refused 'leere Notiz' (With $dispatch @{ InputNotes = '  ' }) 'Notiz'
Expect-Ok 'Trockenlauf von jedem Zweig, ohne Bestätigung' (With $dispatch @{ Ref = 'refs/heads/paket-x'; InputDryRun = 'true'; InputConfirm = '' }) { param($r) $r.DryRun }
Expect-Ok 'Trockenlauf darf Tests überspringen' (With $dispatch @{ Ref = 'refs/heads/paket-x'; InputDryRun = 'true'; InputSkipTests = 'true' }) { param($r) $r.DryRun -and $r.SkipTests }
Expect-Ok 'Tag auf main' @{ EventName = 'push'; Ref = 'refs/tags/v0.7.0'; TagOnMain = 'true' } { param($r) $r.Version -eq '0.7.0' -and -not $r.DryRun -and -not $r.SkipTests -and $r.Notes -eq 'Daedyx Desk 0.7.0' }
Expect-Ok 'Tag auf main, Produkt Tasks' @{ EventName = 'push'; Ref = 'refs/tags/v0.1.0'; TagOnMain = 'true'; Product = 'Tasks' } { param($r) $r.Notes -eq 'Daedyx Tasks 0.1.0' }
Expect-Refused 'Tag nicht auf main' @{ EventName = 'push'; Ref = 'refs/tags/v0.7.0'; TagOnMain = 'false' } 'main'
Expect-Refused 'Tag ohne Prüfergebnis' @{ EventName = 'push'; Ref = 'refs/tags/v0.7.0' } 'main'
Expect-Refused 'Zweig statt Tag bei push' @{ EventName = 'push'; Ref = 'refs/heads/main'; TagOnMain = 'true' } 'Tag'
Expect-Refused 'fremder Auslöser' @{ EventName = 'schedule' } 'Auslöser'

if ($failed) { $failed | ForEach-Object { Write-Host "FEHLER: $_" -ForegroundColor Red }; exit 1 }
Write-Host 'check-release-inputs: alle Proben bestanden' -ForegroundColor Green
