//! Hilfen für Tests (eigene und die anderer Crates über das Merkmal `test-support`). Nie im Produkt-Bau.
use std::ops::Deref;
use std::path::{Path, PathBuf};

/// Wegwerf-Ordner, der sich beim Ende des Tests selbst löscht (Review T5, N5-4). Wer ihn braucht, hält die Wache bis
/// zum Testende in einer Variablen (`let dir = test_dir(..)?;`); `Deref` und `AsRef` geben den Pfad.
#[derive(Debug)]
pub struct TestDir(PathBuf);

impl TestDir {
    pub fn path(&self) -> &Path {
        &self.0
    }

    /// Eine Datei in diesem Ordner, die den Ordner am Leben hält.
    pub fn file(self, name: &str) -> TestFile {
        let path = self.0.join(name);
        TestFile { _dir: self, path }
    }
}

impl Deref for TestDir {
    type Target = Path;
    fn deref(&self) -> &Path {
        &self.0
    }
}

impl AsRef<Path> for TestDir {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestDir {
    /// `remove_dir_all` folgt keiner Verknüpfung und keiner Junction, es entfernt nur den Verweis selbst.
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Pfad einer Datei in einem [`TestDir`]; der Ordner verschwindet erst mit ihr.
#[derive(Debug)]
pub struct TestFile {
    _dir: TestDir,
    path: PathBuf,
}

impl Deref for TestFile {
    type Target = Path;
    fn deref(&self) -> &Path {
        &self.path
    }
}

impl AsRef<Path> for TestFile {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

/// Frischer, leerer Wegwerf-Ordner für einen Test: `<System-Temp>/daedyx-tasks-store-<pid>-<name>`. Nie der
/// Datenordner der App (SICHERHEIT 2.8); der Name enthält die Prozessnummer, damit parallele Läufe sich nicht stören.
#[allow(clippy::disallowed_methods, reason = "SICHERHEIT-AUSNAHMEN A-005: Test-Ordner im System-Temp")]
pub fn test_dir(name: &str) -> std::io::Result<TestDir> {
    let p = std::env::temp_dir().join(format!("daedyx-tasks-store-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&p);
    std::fs::create_dir_all(&p)?;
    Ok(TestDir(p))
}

#[cfg(test)]
mod tests {
    use super::test_dir;

    #[test]
    fn wegwerf_ordner_verschwindet_am_ende() {
        let dir = test_dir("wache").unwrap();
        std::fs::write(dir.join("a.txt"), b"x").unwrap();
        let file = test_dir("wache-datei").unwrap().file("b.db");
        std::fs::write(&file, b"y").unwrap();
        let (kept, parent) = (dir.to_path_buf(), file.parent().unwrap().to_path_buf());
        assert!(kept.exists() && parent.exists());
        drop((dir, file));
        assert!(!kept.exists() && !parent.exists());
    }
}
