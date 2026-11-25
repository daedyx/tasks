//! Zustand der App: Sprache und Texte, gewählte Seite, die Aufgaben-Oberfläche aus `daedyx-tasks-ui`, Hintergrund
//! (Takt, Infobereich, Autostart; `background.rs`) und Feinstellung des Dunkelmodus.
use crate::page::Page;
use crate::settings::Settings;
use crate::texts::texts;
use daedyx_background::Background;
use daedyx_tasks_ui::{Mode, Sel, TaskStore, TasksUi};
use daedyx_ui::i18n::{Language, Texts};
use daedyx_ui::screenshot::Shot;
use daedyx_ui::theme::{self, DarkTuning};
use eframe::egui::{Theme, Ui};

pub struct TasksApp {
    pub language: Language,
    pub texts: Texts,
    pub page: Page,
    pub tasks: TasksUi,
    /// Hintergrund: Fenster, Takt, Benachrichtigung, Autostart, Schalter „Im Infobereich weiterlaufen“.
    pub bg: Background,
    /// Feinstellung des Dunkelmodus (T-F3); `None`: Farben der Tokens. Gilt erst nach [`TasksApp::set_dark_tuning`].
    pub dark_tuning: Option<DarkTuning>,
    dark: Option<bool>,
    started: bool,
    shot: Option<Shot>,
}

impl TasksApp {
    pub fn new(settings: &Settings, store: Option<TaskStore>) -> TasksApp {
        let mut app = TasksApp {
            language: settings.language,
            texts: texts(settings.language),
            page: Page::List,
            tasks: TasksUi::new(store),
            bg: Background::new(settings.language),
            dark_tuning: None,
            dark: settings.dark,
            started: false,
            shot: None,
        };
        // Liste und Board wechselt die Navigationsleiste; der Umschalter der Aufgaben-Oberfläche bleibt aus (T2-1).
        app.tasks.mode_switch = false;
        app.open(settings.page);
        app
    }

    pub fn has_shot(&self) -> bool {
        self.shot.is_some()
    }

    /// Foto nur vom App-Fenster (`DAEDYX_SCREENSHOT`), danach schließt sich das Fenster.
    pub fn with_shot(mut self, shot: Option<Shot>) -> TasksApp {
        self.shot = shot;
        self
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
        self.texts = texts(language);
        self.bg.set_language(language);
    }

    /// Feinstellung des Dunkelmodus ändern oder zurücksetzen (`None`); wirkt sofort.
    pub fn set_dark_tuning(&mut self, ctx: &eframe::egui::Context, tuning: Option<DarkTuning>) {
        self.dark_tuning = tuning;
        theme::set_dark_tuning(ctx, tuning);
    }

    /// Seite wählen. Das Board braucht eine Liste: ohne Auswahl nimmt es die erste.
    pub fn open(&mut self, page: Page) {
        self.page = page;
        match page {
            Page::List => self.tasks.mode = Mode::List,
            Page::Board => {
                if self.tasks.list_id().is_none()
                    && let Some(id) = self.tasks.lists.first().map(|l| l.id.clone())
                {
                    self.tasks.select(Sel::List(id));
                }
                self.tasks.mode = Mode::Board;
            }
            Page::Settings => {}
        }
    }

    /// Welche Seite die Leiste als gewählt zeigt. Wählt man links eine Smart-Liste, fällt das Board auf die Liste
    /// zurück; die Leiste folgt.
    pub fn active(&self) -> Page {
        match self.page {
            Page::Settings => Page::Settings,
            _ if self.tasks.mode == Mode::Board && self.tasks.list_id().is_some() => Page::Board,
            _ => Page::List,
        }
    }

    pub fn frame(&mut self, ui: &mut Ui) {
        let ctx = ui.ctx().clone();
        // Versteckt (Infobereich, `--hidden`): nichts zeichnen (Spike S5, egui zeichnet sonst weiter).
        if self.bg.window.hidden {
            return;
        }
        if !self.started {
            self.started = true;
            if let Some(dark) = self.dark {
                ctx.set_theme(if dark { Theme::Dark } else { Theme::Light });
            }
        }
        if let Some(shot) = &mut self.shot {
            shot.tick(&ctx);
        }
        crate::shell::show(ui, self);
    }
}

impl eframe::App for TasksApp {
    fn logic(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.logic(ctx);
    }

    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        self.frame(ui);
    }
}
