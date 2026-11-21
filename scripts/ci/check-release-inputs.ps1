# Prüft die Eingaben eines Release-Laufs, bevor irgendetwas gebaut oder veröffentlicht wird (Schritt "Eingaben prüfen").
# Echte Veröffentlichung nur von "main" (Hand-Start) oder von einem Tag, dessen Commit auf "main" liegt.
# Der Trockenlauf darf von jedem Zweig gestartet werden. Tests überspringen gibt es nur im Trockenlauf.
# Alles kommt als Text-Parameter (aus Umgebungsvariablen), nichts wird als Skripttext ausgeführt. Gibt Version, Notiz, SkipTests, DryRun zurück.
param(
  [Parameter(Mandatory)][string]$EventName,
  [string]$Ref = '',
  [string]$InputVersion = '',
  [string]$InputNotes = '',
  [string]$InputSkipTests = '',
  [string]$InputDryRun = '',
  [string]$InputConfirm = '',
  [string]$TagOnMain = '',
  [string]$Product = 'Desk'
)
$ErrorActionPreference = 'Stop'

function Test-True([string]$v) { return $v.Trim() -ieq 'true' }

if ($EventName -eq 'push') {
  if ($Ref -notmatch '^refs/tags/v(\d+\.\d+\.\d+)$') { throw "Veröffentlichung per Tag nur mit einem Tag der Form vX.Y.Z (gefunden: '$Ref')." }
  if (-not (Test-True $TagOnMain)) { throw "Der Tag liegt nicht auf dem Zweig main. Nichts wird veröffentlicht." }
  $version = $Matches[1]
  return [pscustomobject]@{ Version = $version; Notes = "Daedyx $Product $version"; SkipTests = $false; DryRun = $false }
}
if ($EventName -ne 'workflow_dispatch') { throw "Unbekannter Auslöser '$EventName'." }

$version = $InputVersion.Trim()
$notes = $InputNotes.Trim()
$skip = Test-True $InputSkipTests
$dry = Test-True $InputDryRun

if ($version -notmatch '^\d+\.\d+\.\d+$') { throw "Ungültige Version: '$version' (erwartet x.y.z)" }
if (-not $notes) { throw "Die Änderungsnotiz fehlt." }
if (-not $dry) {
  if ($Ref -ne 'refs/heads/main') { throw "Echte Veröffentlichung nur vom Zweig main (gestartet von '$Ref'). Für einen Probelauf den Haken Trockenlauf setzen." }
  if ($skip) { throw "Tests überspringen ist nur im Trockenlauf erlaubt." }
  if ($InputConfirm.Trim() -cne $version) { throw "Zur Bestätigung die Version '$version' im Feld Bestätigung erneut eintippen." }
}
return [pscustomobject]@{ Version = $version; Notes = $notes; SkipTests = $skip; DryRun = $dry }
