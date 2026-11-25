//! Takt der Erinnerungen (T4): Jeder Durchgang ruft den geprüften Durchgang des Stores (`reminders::tick_once`) auf
//! derselben Speicher-Datenbank, die die Oberfläche zeigt. Der Store merkt sich jede gemeldete Erinnerung, so kommt
//! jede genau einmal, auch bei bis zu zehn Erinnerungen je Aufgabe. Die Benachrichtigung sagt „Fällig in …“ und ist
//! ohne Titel, solange der Schalter aus ist (T-F6).
use daedyx_background::ticker::{Jobs, Tick};
use daedyx_background::{Notice, NoticeKind};
use daedyx_tasks_store::reminders::{tick_once, unix_now};
use daedyx_tasks_ui::TaskDb;
use rusqlite::Connection;
use std::sync::{Arc, Mutex, MutexGuard};

/// Eine Speicher-Datenbank für Oberfläche und Takt.
pub type SharedConn = Arc<Mutex<Connection>>;

/// Zugang der Oberfläche zur geteilten Datenbank (eine vergiftete Sperre wird übernommen, wie in `daedyx-tasks-ui`).
pub struct SharedDb(pub SharedConn);

impl TaskDb for SharedDb {
    fn lock(&self) -> MutexGuard<'_, Connection> {
        TaskDb::lock(self.0.as_ref())
    }
}

/// Uhr des Takts in Unix-Sekunden (Programm: Systemzeit, Tests: fest).
pub type Now = Box<dyn Fn() -> i64 + Send + Sync>;

pub struct ReminderJobs {
    db: SharedConn,
    zone: String,
    now: Now,
}

impl ReminderJobs {
    pub fn new(db: SharedConn, zone: &str) -> ReminderJobs {
        ReminderJobs::with_clock(db, zone, Box::new(unix_now))
    }

    pub fn with_clock(db: SharedConn, zone: &str, now: Now) -> ReminderJobs {
        ReminderJobs {
            db,
            zone: zone.to_owned(),
            now,
        }
    }
}

impl Jobs for ReminderJobs {
    fn run(&self) -> Tick {
        let mut notices = Vec::new();
        tick_once(&self.db, (self.now)(), &self.zone, |r| {
            notices.push(Notice {
                kind: NoticeKind::TaskLead {
                    minutes_before: r.minutes_before,
                },
                title: r.title.clone(),
                time: String::new(),
            });
        });
        Tick {
            notices,
            sync_due: 0,
        }
    }
}
