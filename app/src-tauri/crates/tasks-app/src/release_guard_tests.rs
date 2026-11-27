//! Gegenproben mit Windows PowerShell (Review T7v): Der Secret-Wächter aus `ci.yml` schlägt bei `release.yml` nicht an,
//! aber bei jedem Zugriff (M-T7v-1, mit denselben Mustern wie `secret_hits`). `make-entry.ps1` lehnt Notizen ab, die
//! der Kern ablehnt (unsichtbare Format- und Steuerzeichen, M-T7v-2), damit der Lauf früh und klar stoppt.
use super::flow::{make_entry, powershell};
use super::{HARMLESS, RELEASE_SECRET_LINE, SECRET_SAMPLES, repo_root, repo_text, secret_hits};
use crate::test_support::tmp;
use daedyx_sys::RunEnd;
use daedyx_updater::daedyx_update::policy::WORKFLOW;
use std::path::Path;

/// Zeilennummern, in denen die Muster aus `ci.yml` (Zeile `$releaseMuster = …`) in `file` anschlagen.
fn ci_hits(dir: &Path, file: &Path) -> Vec<usize> {
    let ci = repo_text(".github/workflows/ci.yml");
    let line = ci
        .lines()
        .map(str::trim)
        .find(|l| *l == RELEASE_SECRET_LINE)
        .unwrap();
    let script = dir.join("muster.ps1");
    let body = format!(
        "{line}\n$t = @(Select-String -LiteralPath $args[0] -CaseSensitive -Pattern $releaseMuster)\n\
         Set-Content -LiteralPath $args[1] -Value (($t | ForEach-Object LineNumber) -join ',')\n"
    );
    std::fs::write(&script, body).unwrap();
    let out = dir.join("treffer.txt");
    let args = [file.to_str().unwrap(), out.to_str().unwrap()];
    assert_eq!(powershell(&script, &args), RunEnd::Exited(Some(0)));
    let text = std::fs::read_to_string(out).unwrap();
    text.trim()
        .split(',')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse().unwrap())
        .collect()
}

#[test]
fn secret_waechter_der_ci_trifft_genau_wie_der_test() {
    let dir = tmp("t7v-muster");
    for (name, text) in [("zugriffe.yml", SECRET_SAMPLES), ("harmlos.yml", HARMLESS)] {
        let file = dir.join(name);
        std::fs::write(&file, text).unwrap();
        assert_eq!(ci_hits(&dir, &file), secret_hits(text), "{name}");
    }
    assert_eq!(
        ci_hits(&dir, &repo_root().join(WORKFLOW)),
        Vec::<usize>::new()
    );
}

#[test]
fn make_entry_lehnt_notizen_ab_die_der_kern_ablehnt() {
    let dir = tmp("t7v-notiz");
    let setup = dir.join("daedyx-tasks-9.9.9-setup.exe");
    std::fs::write(&setup, b"erfunden").unwrap();
    let ok = "Zeile mit\tTab\nzweite Zeile, \u{e4}\u{f6}\u{fc} \u{1F600}";
    assert_eq!(
        make_entry(&dir, &setup, "daedyx-tasks", ok),
        RunEnd::Exited(Some(0))
    );
    // Je ein Zeichen aus den Bereichen von `text::plain` im Kern: C0, C1, weiches Trennzeichen, Nullbreite,
    // Richtungsmarke, Zeilentrenner, Wortverbinder, BOM, Tag-Zeichen (außerhalb der BMP), einzelnes \r.
    for bad in [
        "\u{7}",
        "\u{85}",
        "\u{ad}",
        "\u{200b}",
        "\u{202e}",
        "\u{2028}",
        "\u{2060}",
        "\u{feff}",
        "\u{e0041}",
        "a\rb",
    ] {
        let end = make_entry(&dir, &setup, "daedyx-tasks", &format!("Notiz {bad} Ende"));
        assert_eq!(
            end,
            RunEnd::Exited(Some(1)),
            "{:?}",
            bad.escape_unicode().to_string()
        );
    }
}
