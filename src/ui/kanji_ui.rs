use {
    crate::appstate::{AppState, UiState},
    egui_sfml::egui,
};

pub fn kanji_ui(ui: &mut egui::Ui, app: &mut AppState) {
    if ui.link("Back").clicked() {
        app.ui_state = UiState::Input;
    }
    egui::ScrollArea::vertical().show_rows(ui, 20.0, app.kanji_db.kanji.len(), |ui, range| {
        ui.set_min_width(600.0);
        for kanji in &app.kanji_db.kanji[range] {
            ui.horizontal(|ui| {
                for c in kanji.chars {
                    ui.label(c);
                }
                ui.label(kanji.meaning);
                for &reading in &kanji.readings {
                    ui.label(reading);
                }
            });
        }
    });
}
