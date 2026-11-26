//! Verbindung der App zum Hintergrund (T4): Ereignisse vor jedem Bild anwenden, auch bei verstecktem Fenster; vor dem
//! Beenden den Entwurf im Detailbereich übernehmen; den Takt auf der Speicher-Datenbank starten.
use crate::app::TasksApp;
use crate::reminders::{ReminderJobs, SharedConn};
use daedyx_background::Event;
use eframe::egui::Context;
use std::sync::Arc;
use std::time::Duration;

impl TasksApp {
    /// Läuft vor jedem Bild und bei verstecktem Fenster allein (`eframe::App::logic`).
    pub fn logic(&mut self, ctx: &Context) {
        self.save_prefs(ctx);
        // Update (U2): Meldungen holen; ein geprüftes Setup startet und beendet die App wie „Beenden“.
        self.updater.poll();
        if let Some(prepared) = self.updater.take_ready() {
            self.install_update(prepared);
        }
        if let Some(setup) = self.update_probe.take() {
            self.run_update_probe(setup);
        }
        // Bildschirmfoto-Modus: Das Foto schließt das Fenster; das soll wirklich beenden, nicht verstecken.
        if self.has_shot() {
            self.bg.window.quitting = true;
        }
        self.bg.logic(ctx);
        if self.bg.window.quit_wanted {
            self.bg.window.quit_wanted = false;
            let cmds = if self.tasks.commit_draft(&self.texts) {
                self.bg.window.finish_quit()
            } else {
                // Entwurf ließ sich nicht übernehmen: offen bleiben, Fenster zeigen, Fehler steht am Feld.
                self.bg.window.apply(&Event::Show)
            };
            for cmd in cmds {
                ctx.send_viewport_cmd(cmd);
            }
        }
    }

    /// „Im Infobereich weiterlaufen“ (Standard an): aus heißt, Schließen beendet die App.
    pub fn set_keep_running(&mut self, on: bool) {
        self.bg.window.set_keep_running(on);
    }

    /// Startet den Takt der Erinnerungen auf der Datenbank der Oberfläche.
    pub fn start_reminders(&mut self, every: Duration, db: SharedConn, zone: &str) {
        self.bg
            .start_ticker(every, Arc::new(ReminderJobs::new(db, zone)));
    }
}
