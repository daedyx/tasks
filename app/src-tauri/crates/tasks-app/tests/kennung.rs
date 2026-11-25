//! Plan 4.2: Im Vorschau-Programm (ohne Merkmal `produkt`) steht die echte Kennung `de.daedyx.tasks` nirgends, nur
//! die der Vorschau `de.daedyx.tasks.vorschau`. Geprüft an den Bytes des gebauten Programms.
const EXE: &str = env!("CARGO_BIN_EXE_daedyx-tasks-vorschau");

#[test]
fn vorschau_programm_enthaelt_die_echte_kennung_nicht() {
    let bytes = std::fs::read(EXE).unwrap();
    let (id, suffix) = (b"de.daedyx.tasks".as_slice(), b".vorschau".as_slice());
    let mut found = 0;
    for (at, window) in bytes.windows(id.len()).enumerate() {
        if window == id {
            found += 1;
            let rest = &bytes[at + id.len()..];
            assert!(rest.starts_with(suffix), "echte Kennung an Stelle {at}");
        }
    }
    assert!(found >= 1, "Vorschau-Kennung fehlt im Programm");
}
