use crate::{
    appstate::{AppState, UiState},
    egui,
};

#[derive(Default)]
pub struct KanjiUiState {
    filter_string: String,
    tab: Tab = Tab::Kanji,
}

#[derive(PartialEq)]
enum Tab {
    Kanji,
    Radicals,
}

pub fn kanji_ui(ui: &mut egui::Ui, app: &mut AppState) {
    ui.horizontal(|ui| {
        if ui.link("Back (Esc)").clicked() || ui.input(|inp| inp.key_pressed(egui::Key::Escape)) {
            app.ui_state = UiState::Input;
        }
        ui.selectable_value(&mut app.kanji_ui_state.tab, Tab::Kanji, "Kanji");
        ui.selectable_value(&mut app.kanji_ui_state.tab, Tab::Radicals, "Radicals");
        ui.add(
            egui::TextEdit::singleline(&mut app.kanji_ui_state.filter_string).hint_text("Filter"),
        );
    });
    ui.separator();
    match app.kanji_ui_state.tab {
        Tab::Kanji => kanji_tab(ui, app),
        Tab::Radicals => radicals_tab(ui),
    }
}

pub fn kanji_tab(ui: &mut egui::Ui, app: &mut AppState) {
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

pub fn radicals_tab(ui: &mut egui::Ui) {
    for pair in crate::radicals::PAIRS {
        ui.horizontal(|ui| {
            ui.label(pair.ch.to_string());
            ui.label(pair.name);
        });
    }
}
