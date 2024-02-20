use {
    crate::appstate::{AppState, UiState},
    egui_sfml::egui,
};

#[derive(Default)]
pub struct KanjiUiState {
    filter_string: String,
}

pub fn kanji_ui(ui: &mut egui::Ui, app: &mut AppState) {
    ui.horizontal(|ui| {
        if ui.link("Back").clicked() {
            app.ui_state = UiState::Input;
        }
        ui.add(
            egui::TextEdit::singleline(&mut app.kanji_ui_state.filter_string).hint_text("Filter"),
        );
    });
    ui.separator();
    let mut filtered = app.kanji_db.kanji.clone();
    if !app.kanji_ui_state.filter_string.is_empty() {
        filtered.retain(|kanji| kanji.meaning.contains(&app.kanji_ui_state.filter_string));
    }
    egui::ScrollArea::vertical().show_rows(ui, 20.0, filtered.len(), |ui, range| {
        ui.set_min_width(600.0);
        for kanji in &filtered[range] {
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
