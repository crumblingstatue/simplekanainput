use {
    crate::{
        appstate::{AppState, UiState},
        egui,
    },
    ids_rust::FilterLevel,
};

#[derive(Default)]
pub struct KanjiUiState {
    filter_string: String,
    tab: Tab = Tab::Kanji,
    adv_args: ids_rust::SearchArgs = default_adv_args(),
    adv_results: Vec<ids_rust::SearchResult>,
    adv_input_buf: String,
}

const fn default_adv_args() -> ids_rust::SearchArgs {
    ids_rust::SearchArgs {
        reverse: false,
        simple: true,
        lite: true,
        filter_level: FilterLevel::JoyoPlus,
        input: None,
    }
}

#[derive(PartialEq)]
enum Tab {
    Kanji,
    Radicals,
    Advanced,
}

pub fn kanji_ui(ui: &mut egui::Ui, app: &mut AppState) {
    ui.horizontal(|ui| {
        if ui.link("Back (Esc)").clicked() || ui.input(|inp| inp.key_pressed(egui::Key::Escape)) {
            app.ui_state = UiState::Input;
        }
        ui.selectable_value(&mut app.kanji_ui_state.tab, Tab::Kanji, "Kanji");
        ui.selectable_value(&mut app.kanji_ui_state.tab, Tab::Radicals, "Radicals");
        ui.selectable_value(&mut app.kanji_ui_state.tab, Tab::Advanced, "Advanced");
        ui.add(
            egui::TextEdit::singleline(&mut app.kanji_ui_state.filter_string).hint_text("Filter"),
        );
    });
    ui.separator();
    match app.kanji_ui_state.tab {
        Tab::Kanji => kanji_tab(ui, app),
        Tab::Radicals => radicals_tab(ui),
        Tab::Advanced => advanced_tab(ui, app),
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
                    if ui
                        .add(egui::Label::new(c).sense(egui::Sense::click()))
                        .clicked()
                    {
                        ui.ctx().copy_text(c.to_string());
                    }
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
            let s = pair.ch.to_string();
            if ui
                .add(egui::Label::new(&s).sense(egui::Sense::click()))
                .clicked()
            {
                ui.ctx().copy_text(s);
            }
            ui.label(pair.name);
        });
    }
}

pub fn advanced_tab(ui: &mut egui::Ui, app: &mut AppState) {
    let args = &mut app.kanji_ui_state.adv_args;
    let mut any_changed = false;
    any_changed ^= ui.checkbox(&mut args.reverse, "reverse").changed();
    any_changed ^= ui.checkbox(&mut args.simple, "simple").changed();
    any_changed ^= ui.checkbox(&mut args.lite, "lite").changed();
    ui.horizontal(|ui| {
        ui.label("input");
        any_changed ^= ui
            .text_edit_singleline(&mut app.kanji_ui_state.adv_input_buf)
            .changed();
    });
    egui::ComboBox::new("filter_combo", "Filter")
        .selected_text(format!("{:?}", args.filter_level))
        .show_ui(ui, |ui| {
            any_changed ^= ui
                .selectable_value(&mut args.filter_level, FilterLevel::All, "All")
                .clicked();
            any_changed ^= ui
                .selectable_value(&mut args.filter_level, FilterLevel::JoyoPlus, "JoyoPlus")
                .clicked();
            any_changed ^= ui
                .selectable_value(
                    &mut args.filter_level,
                    FilterLevel::KanjiDicPlus,
                    "KanjiDicPlus",
                )
                .clicked();
            any_changed ^= ui
                .selectable_value(&mut args.filter_level, FilterLevel::Media, "Media")
                .clicked();
        });
    if any_changed {
        args.input = (!app.kanji_ui_state.adv_input_buf.is_empty())
            .then(|| app.kanji_ui_state.adv_input_buf.clone());
        app.kanji_ui_state.adv_results = app.ids_kanji_data.search(args.clone());
    }
    ui.separator();
    let mut prev_strokes = 0;
    egui::ScrollArea::vertical()
        .auto_shrink(false)
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                for result in &app.kanji_ui_state.adv_results {
                    if result.strokes != prev_strokes {
                        ui.end_row();
                        ui.label(format!("{} strokes", result.strokes));
                        ui.end_row();
                    }
                    prev_strokes = result.strokes;
                    if ui
                        .add(
                            egui::Label::new(
                                egui::RichText::new(result.kanji.to_string()).size(48.0),
                            )
                            .sense(egui::Sense::click()),
                        )
                        .clicked()
                    {
                        ui.ctx().copy_text(result.kanji.to_string());
                    }
                }
            });
        });
}
