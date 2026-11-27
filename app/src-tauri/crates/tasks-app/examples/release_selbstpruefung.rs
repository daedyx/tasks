//! Selbstprüfung im Release-Lauf (`release.yml`, Review T7v M-T7v-2): prüft Setup, `entry.json` und Bündel in
//! `<Ordner>` mit dem echten GitHub-Prüfer der App (eingebaute Vertrauenswurzel, Regeln des Kerns), bevor etwas
//! veröffentlicht wird. Nur mit dem Merkmal `produkt` passt die App-Kennung; ohne lehnt der Prüfweg ab.
//! Aufruf: `cargo run --release -p daedyx-tasks-app --features produkt --example release_selbstpruefung --
//! <Ordner> <Version> <Arbeitsordner>`. Endet mit Fehlercode, wenn die App das Release ablehnen würde.
use daedyx_tasks_app::{release_check, update};
use daedyx_updater::daedyx_update::attest::GithubAttestation;
use std::path::Path;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [dir, version, work] = args.as_slice() else {
        return Err("Aufruf: <Ordner> <Version> <Arbeitsordner>".into());
    };
    let policy = update::policy().ok_or("keine Update-Regel für dieses System")?;
    let github = GithubAttestation::new(&policy).map_err(|e| format!("Prüfer: {e:?}"))?;
    release_check::verify(Path::new(dir), version, &github, Path::new(work))
        .map(|_| ())
        .map_err(|r| format!("Die App würde dieses Release ablehnen: {r:?}"))
}
