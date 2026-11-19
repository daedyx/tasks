# Lädt daedyx-catalog.exe (Katalog-Werkzeug des Daedyx Launchers) aus dem öffentlichen Repository daedyx/releases
# und prüft die SHA-256-Summe. Die Pipeline benutzt nur dieses geprüfte Programm.
param(
  [Parameter(Mandatory)][string]$Destination,
  [string]$BaseUrl = "https://github.com/daedyx/releases/releases/download/tools"
)
$ErrorActionPreference = "Stop"
$exe = "$BaseUrl/daedyx-catalog.exe"
$sum = "$BaseUrl/daedyx-catalog.exe.sha256"
Invoke-WebRequest -Uri $exe -OutFile $Destination -UseBasicParsing
# Die Datei kommt als application/octet-stream: Inhalt über eine Datei lesen, nicht über .Content (wäre byte[]).
$sumFile = [System.IO.Path]::GetTempFileName()
try {
  Invoke-WebRequest -Uri $sum -OutFile $sumFile -UseBasicParsing
  $expected = ([System.IO.File]::ReadAllText($sumFile).Trim() -split '\s+')[0].ToLowerInvariant()
} finally { Remove-Item $sumFile -ErrorAction SilentlyContinue }
if ($expected -notmatch '^[0-9a-f]{64}$') { throw "Die Prüfsummen-Datei hat ein ungültiges Format." }
$actual = (Get-FileHash $Destination -Algorithm SHA256).Hash.ToLowerInvariant()
if ($actual -ne $expected) {
  Remove-Item $Destination -ErrorAction SilentlyContinue
  throw "Die Prüfsumme von daedyx-catalog.exe stimmt nicht (erwartet $expected, erhalten $actual)."
}
Write-Output "daedyx-catalog.exe geladen und geprüft."
