# Ordner der Installer-Vorlage aus daedyx-build (Tasks T1b): über `cargo metadata` aus dem Cargo-Zwischenspeicher,
# also genau die Fassung aus Cargo.lock. Ohne -Offline darf cargo den Stand holen, mit -Offline nie.
# Gelesen wird nur die Zeile mit dem Pfad (Windows PowerShell 5.1 liest große JSON-Ausgaben nicht zuverlässig).
function Get-InstallerVorlage([string]$Workspace, [switch]$Offline) {
    $cargoArgs = @("metadata", "--format-version", "1", "--locked", "--manifest-path", (Join-Path $Workspace "Cargo.toml"))
    if ($Offline) { $cargoArgs += "--offline" }
    $json = & cargo @cargoArgs
    if ($LASTEXITCODE -ne 0) { throw "cargo metadata fehlgeschlagen ($LASTEXITCODE)" }
    # Pfade stehen im JSON mit doppelten Backslashes.
    $m = [regex]::Match(($json -join ""), '"manifest_path":"([^"]*[\\/]daedyx-build[\\/]+Cargo\.toml)"')
    if (-not $m.Success) { throw "daedyx-build nicht im Baum (Bau-Abhängigkeit von tasks-app)" }
    $manifest = $m.Groups[1].Value.Replace('\\', '\')
    $dir = Join-Path (Split-Path $manifest -Parent) "vorlage\installer"
    if (-not (Test-Path (Join-Path $dir "app-vorschau.nsi"))) { throw "Installer-Vorlage fehlt: $dir" }
    return $dir
}
