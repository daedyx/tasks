//! Takt der Erinnerungen (T4): jede Erinnerung genau einmal, auch bei mehreren Erinnerungen derselben Aufgabe;
//! Benachrichtigung standardmäßig neutral ohne Titel (T-F6). Speicher-Datenbank, feste Uhr, erfundene Aufgabe.
use crate::reminders::{ReminderJobs, SharedConn, SharedDb, minutes_until};
use crate::seed;
use daedyx_background::notify::Recorder;
use daedyx_background::ticker::{Jobs, NoticeStyle, run_once};
use daedyx_background::{NoticeKind, Sink};
use daedyx_tasks_ui::{Clock, TaskStore};
use daedyx_ui::i18n::Language;
use serde_json::{Map, Value, json};
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};

/// Fälligkeit der erfundenen Aufgabe: 18.11.2026, 12:00 UTC.
const DUE: i64 = 1_795_003_200;

/// Datenbank mit einer Aufgabe „Wochenmarkt“, fällig um `DUE`, Erinnerungen 10 und 5 Minuten vorher.
fn db_with_two_alarms() -> SharedConn {
    let db = seed::shared().unwrap();
    let store = TaskStore::on(
        Box::new(SharedDb(Arc::clone(&db))),
        chrono_tz::Europe::Berlin,
        Clock::System,
    );
    let list = store
        .create_list("Einkauf", &daedyx_tasks_ui::colors::stored(0))
        .unwrap();
    let mut f = Map::new();
    f.insert("title".into(), json!("Wochenmarkt"));
    f.insert("due".into(), Value::String(DUE.to_string()));
    f.insert("due_has_time".into(), json!(true));
    let task = store.create(&list.id, None, &f).unwrap();
    {
        let conn = db.lock().unwrap();
        daedyx_tasks_core::edit_alarms::set_alarms(&conn, &task.id, &[10, 5], 0).unwrap();
    }
    db
}

fn jobs(db: &SharedConn, now: &Arc<AtomicI64>) -> ReminderJobs {
    let clock = Arc::clone(now);
    ReminderJobs::with_clock(
        Arc::clone(db),
        "Europe/Berlin",
        Box::new(move || clock.load(Ordering::Relaxed)),
    )
}

fn leads(j: &ReminderJobs) -> Vec<i64> {
    j.run()
        .notices
        .into_iter()
        .map(|n| match n.kind {
            NoticeKind::TaskLead { minutes_until } => minutes_until,
            other => panic!("unerwartete Art {other:?}"),
        })
        .collect()
}

#[test]
fn jede_erinnerung_genau_einmal_auch_bei_zweien_je_aufgabe() {
    let db = db_with_two_alarms();
    let now = Arc::new(AtomicI64::new(DUE - 11 * 60));
    let j = jobs(&db, &now);
    assert!(leads(&j).is_empty(), "noch nichts fällig");
    now.store(DUE - 10 * 60, Ordering::Relaxed);
    assert_eq!(leads(&j), vec![10]);
    assert!(leads(&j).is_empty(), "nicht zweimal");
    now.store(DUE - 5 * 60, Ordering::Relaxed);
    assert_eq!(
        leads(&j),
        vec![5],
        "die zweite Erinnerung, die erste nicht erneut"
    );
    assert!(leads(&j).is_empty());
    now.store(DUE + 60, Ordering::Relaxed);
    assert!(leads(&j).is_empty(), "alles gemeldet");
}

#[test]
fn verpasste_erinnerungen_derselben_aufgabe_eine_meldung_mit_echter_restzeit() {
    // Review T4, N4-1: PC schlief über beide Erinnerungen hinweg; beim Aufwachen 2 Minuten vor der Fälligkeit kommt
    // eine Meldung „Fällig in 2 Minuten“, nicht „10“ und „5“. Beide gelten danach als gemeldet.
    let db = db_with_two_alarms();
    let now = Arc::new(AtomicI64::new(DUE - 2 * 60));
    let j = jobs(&db, &now);
    assert_eq!(leads(&j), vec![2]);
    assert!(leads(&j).is_empty(), "die andere still als gemeldet");
}

#[test]
fn nachgeholt_nach_der_faelligkeit_sagt_seit_wann() {
    let db = db_with_two_alarms();
    let now = Arc::new(AtomicI64::new(DUE + 7 * 60 + 10));
    let j = jobs(&db, &now);
    assert_eq!(leads(&j), vec![-7]);
    let rec = Recorder::default();
    let (sink, _rx) = Sink::new();
    now.store(DUE + 20, Ordering::Relaxed);
    let db2 = db_with_two_alarms();
    let j2 = jobs(&db2, &now);
    run_once(&j2, &rec, &sink, NoticeStyle::new(Language::De));
    assert_eq!(rec.sent().unwrap().last().unwrap().1, "Jetzt fällig");
}

#[test]
fn restzeit_rechnet_geprueft() {
    let r = |fire_at: i64, minutes_before: i64| daedyx_tasks_store::reminders::Reminder {
        task_id: "a".into(),
        title: "Gießkanne füllen".into(),
        fire_at,
        minutes_before,
        due_has_time: true,
    };
    assert_eq!(minutes_until(&r(DUE - 600, 10), DUE - 600), 10);
    assert_eq!(minutes_until(&r(DUE - 600, 10), DUE - 29), 0);
    assert_eq!(minutes_until(&r(DUE - 600, 10), DUE + 31), -1);
    assert_eq!(
        minutes_until(&r(i64::MAX, 10), 0),
        0,
        "Überlauf ergibt jetzt"
    );
    assert_eq!(minutes_until(&r(0, i64::MAX), 0), 0);
    assert_eq!(minutes_until(&r(0, 0), i64::MIN), 0);
}

#[test]
fn uhr_ausserhalb_des_bereichs_meldet_nichts() {
    let db = db_with_two_alarms();
    for t in [i64::MIN, -1, 253_402_300_800, i64::MAX] {
        let j = jobs(&db, &Arc::new(AtomicI64::new(t)));
        assert!(leads(&j).is_empty(), "{t}");
    }
}

#[test]
fn benachrichtigung_ohne_titel_standard() {
    let db = db_with_two_alarms();
    let now = Arc::new(AtomicI64::new(DUE - 10 * 60));
    let j = jobs(&db, &now);
    let rec = Recorder::default();
    let (sink, _rx) = Sink::new();
    run_once(&j, &rec, &sink, NoticeStyle::new(Language::De));
    let sent = rec.sent().unwrap().clone();
    assert_eq!(
        sent,
        vec![(
            "Erinnerung an eine Aufgabe".to_owned(),
            "Fällig in 10 Minuten".to_owned()
        )]
    );
    now.store(DUE - 5 * 60, Ordering::Relaxed);
    let mut style = NoticeStyle::new(Language::De);
    style.show_titles = true;
    run_once(&j, &rec, &sink, style);
    let last = rec.sent().unwrap().last().cloned().unwrap();
    assert_eq!(last.0, "Aufgabe: Wochenmarkt", "Titel nur per Schalter");
    assert_eq!(last.1, "Fällig in 5 Minuten");
}
