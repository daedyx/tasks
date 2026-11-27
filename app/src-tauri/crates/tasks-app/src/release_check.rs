//! Selbstprüfung eines Releases vor dem Veröffentlichen (Review T7v, M-T7v-2): Setup, `entry.json` und Bündel aus dem
//! Ordner des Release-Laufs gehen genau den Weg, den die App beim Update geht (`check::check` und
//! `check::fetch_setup` mit `update::policy()`). Nur der Transport ist lokal: Er liefert die Dateien unter den
//! Adressen, unter denen die App sie abrufen würde. Der Release-Lauf ruft das mit dem echten GitHub-Prüfer auf
//! (`examples/release_selbstpruefung.rs`), der Test `release_flow_tests.rs` mit einer Attrappe.
use crate::update::policy;
use daedyx_updater::daedyx_update::attest::Provenance;
use daedyx_updater::daedyx_update::check::{self, BUNDLE_FILE, ENTRY_FILE, Outcome};
use daedyx_updater::daedyx_update::fetch::{HttpsUrl, Reply, Transport, TransportError};
use daedyx_updater::daedyx_update::reason::Blocked;
use daedyx_updater::daedyx_update::version::{self, Version};
use std::path::{Path, PathBuf};

/// Liefert Dateien aus `dir`: Eintrag und Bündel unter `latest/download`, Pakete unter `download/v<Version>/`.
struct LocalRelease {
    dir: PathBuf,
    latest: String,
    download: String,
}

impl LocalRelease {
    fn file_for(&self, url: &str) -> Option<PathBuf> {
        let latest = url
            .strip_prefix(&self.latest)
            .filter(|n| *n == ENTRY_FILE || *n == BUNDLE_FILE);
        let name = latest.or_else(|| url.strip_prefix(&self.download))?;
        let plain = !name.is_empty()
            && !name.contains("..")
            && name
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b));
        plain.then(|| self.dir.join(name))
    }
}

impl Transport for LocalRelease {
    fn get(&mut self, url: &HttpsUrl) -> Result<Reply<'_>, TransportError> {
        let file = self
            .file_for(url.as_str())
            .and_then(|p| std::fs::File::open(p).ok());
        Ok(match file {
            Some(f) => Reply::Body {
                length: f.metadata().ok().map(|m| m.len()),
                reader: Box::new(f),
            },
            None => Reply::Status(404),
        })
    }
}

/// Warum die App das Release ablehnen würde (nur Diagnose für den Release-Lauf, ohne Inhalte; keine Oberfläche).
#[derive(Debug)]
pub enum Rejected {
    /// Für dieses System gibt es keine Update-Regel.
    NoPolicy,
    /// Die eingegebene Version ist keine reine `X.Y.Z`.
    BadVersion,
    /// Die Suche bietet nichts an (Grund des Kerns).
    Check(Outcome),
    /// `entry.json` nennt eine andere Version.
    WrongVersion(Version),
    /// Der Arbeitsordner ließ sich nicht anlegen.
    Work(std::io::ErrorKind),
    /// Der Download wurde abgelehnt (Grund des Kerns).
    Fetch(Blocked),
}

/// Prüft das Release in `dir` wie die App und lädt das Setup nach `work`. `Ok` mit dem Pfad des geprüften Setups
/// nur, wenn die App die Version `version` anbieten und das Setup annehmen würde.
pub fn verify(
    dir: &Path,
    version: &str,
    provenance: &dyn Provenance,
    work: &Path,
) -> Result<PathBuf, Rejected> {
    let policy = policy().ok_or(Rejected::NoPolicy)?;
    let want = version::parse(version).map_err(|_| Rejected::BadVersion)?;
    let mut server = LocalRelease {
        dir: dir.to_owned(),
        latest: policy.latest_asset_url(""),
        download: format!("https://github.com{}", policy.release_path_prefix(&want)),
    };
    let none = Version::new(0, 0, 0);
    let offer = match check::check(&mut server, provenance, &policy, &none, None) {
        Outcome::Available(offer) => offer,
        other => return Err(Rejected::Check(other)),
    };
    if offer.entry().version != want {
        return Err(Rejected::WrongVersion(offer.entry().version.clone()));
    }
    std::fs::create_dir_all(work).map_err(|e| Rejected::Work(e.kind()))?;
    let mut progress = |_: u64, _: u64| {};
    check::fetch_setup(
        &mut server,
        provenance,
        &policy,
        &offer,
        work,
        None,
        &mut progress,
    )
    .map_err(Rejected::Fetch)
}
