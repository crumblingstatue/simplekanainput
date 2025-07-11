use {
    crate::{
        appstate::{AppState, UiState},
        egui,
        kanji::Kanji,
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
        crate::ui::show_menu_button(app, ui);
    });
    ui.separator();
    match app.kanji_ui_state.tab {
        Tab::Kanji => kanji_tab(ui, app),
        Tab::Radicals => radicals_tab(ui, &app.kanji_ui_state),
        Tab::Advanced => advanced_tab(ui, app),
    }
}

pub fn kanji_tab(ui: &mut egui::Ui, app: &mut AppState) {
    let mut filtered = app.kanji_db.kanji.clone();
    if !app.kanji_ui_state.filter_string.is_empty() {
        filtered.retain(|kanji| kanji.meaning.contains(&app.kanji_ui_state.filter_string));
    }
    egui::ScrollArea::vertical().auto_shrink(false).show_rows(
        ui,
        20.0,
        filtered.len(),
        |ui, range| {
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
        },
    );
}

pub fn radicals_tab(ui: &mut egui::Ui, kan_ui: &KanjiUiState) {
    egui::ScrollArea::vertical()
        .auto_shrink(false)
        .show(ui, |ui| {
            for (i, rad) in crate::radicals::RADICALS.iter().enumerate() {
                let filt = &kan_ui.filter_string;
                if !filt.is_empty() {
                    let hir = crate::conv::romaji_to_kana(filt, &crate::kana::HIRAGANA);
                    let kat = crate::conv::romaji_to_kana(filt, &crate::kana::KATAKANA);
                    if !rad
                        .common_names
                        .iter()
                        .any(|name| name.contains(&hir) || name.contains(&kat))
                    {
                        continue;
                    }
                }
                ui.horizontal(|ui| {
                    ui.label((i + 1).to_string());
                    for ch in rad.chars {
                        let s = ch.to_string();
                        if ui
                            .add(egui::Label::new(&s).sense(egui::Sense::click()))
                            .clicked()
                        {
                            ui.ctx().copy_text(s);
                        }
                    }
                    for &name in rad.common_names {
                        ui.label(name);
                    }
                });
            }
        });
}

pub fn advanced_tab(ui: &mut egui::Ui, app: &mut AppState) {
    let args = &mut app.kanji_ui_state.adv_args;
    let mut any_changed = false;
    ui.horizontal(|ui| {
        any_changed ^= ui.checkbox(&mut args.reverse, "reverse").changed();
        any_changed ^= ui.checkbox(&mut args.simple, "simple").changed();
        any_changed ^= ui.checkbox(&mut args.lite, "lite").changed();
    });
    ui.horizontal(|ui| {
        ui.label("component(s)");
        any_changed ^= ui
            .text_edit_singleline(&mut app.kanji_ui_state.adv_input_buf)
            .changed();
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
                        .on_hover_ui(|ui| radical_hover_ui(ui, result.kanji, &app.kanji_db.kanji))
                        .clicked()
                    {
                        ui.ctx().copy_text(result.kanji.to_string());
                    }
                }
            });
        });
}

fn radical_hover_ui(ui: &mut egui::Ui, rad: char, kanji_db: &[Kanji]) {
    for (rad_idx, db_rad) in crate::radicals::RADICALS.iter().enumerate() {
        if db_rad.chars.contains(&rad) {
            ui.horizontal(|ui| {
                ui.heading(format!("Radical {}", rad_idx + 1));
                for ch in db_rad.chars {
                    ui.label(ch.to_string());
                }
            });
            ui.separator();
            ui.label("name(s)");
            ui.horizontal(|ui| {
                for &name in db_rad.names {
                    ui.label(name);
                }
            });
            ui.label("common name(s)");
            ui.horizontal(|ui| {
                for &name in db_rad.common_names {
                    ui.label(name);
                }
            });
            ui.separator();
        }
    }
    for kanji in kanji_db {
        if kanji.chars.iter().any(|k| k.starts_with(rad)) {
            ui.horizontal(|ui| {
                for char in kanji.chars {
                    ui.label(char);
                }
            });
            ui.label(kanji.meaning);
            ui.horizontal_wrapped(|ui| {
                for &reading in &kanji.readings {
                    ui.label(reading);
                    ui.label("、");
                }
            });
            ui.separator();
        }
    }
}
