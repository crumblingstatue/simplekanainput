use {
    super::dict_en_ui,
    crate::{
        appstate::{AppState, CachedSuggestions, UiState},
        conv::{self, decompose, Intp, IntpMap},
        kana::{HIRAGANA, KATAKANA},
        segment::segment,
    },
    egui_sfml::egui::{self, Color32, Modifiers},
};

pub fn input_ui(ui: &mut egui::Ui, app: &mut AppState) {
    let mut repopulate_suggestion_cache = false;
    let mut copy_jap_clicked = false;
    let mut segmentation_count_changed = false;
    let (ctrl_enter, f1, f2, f3, f5, f6, f7, esc) = ui.input_mut(|inp| {
        (
            inp.consume_key(Modifiers::CTRL, egui::Key::Enter),
            inp.key_pressed(egui::Key::F1),
            inp.key_pressed(egui::Key::F2),
            inp.key_pressed(egui::Key::F3),
            inp.key_pressed(egui::Key::F5),
            inp.key_pressed(egui::Key::F6),
            inp.key_pressed(egui::Key::F7),
            inp.key_pressed(egui::Key::Escape),
        )
    });
    if esc {
        app.hide_requested = true;
    }
    ui.horizontal(|ui| {
        if ui.button("[F1] 📖 Dict").clicked() || f1 {
            app.ui_state = UiState::Dict;
            app.dict_ui_state.focus_textinput = true;
        }
        if ui.button("[F2] 📋 Copy").clicked() || f2 {
            copy_jap_clicked = true;
        }
        if ui.button("[F3] 🗑 Clear attr").clicked() || f3 {
            app.intp.clear();
        }
        if ui.button("Kanji dict").clicked() {
            app.ui_state = UiState::Kanji;
        }
        if ui.button("Quit").clicked() {
            app.quit_requested = true;
        }
    });
    ui.separator();
    egui::ScrollArea::vertical()
        .max_height(app.half_dims.h.into())
        .id_source("romaji_scroll")
        .show(ui, |ui| {
            let re = ui.add(
                egui::TextEdit::multiline(&mut app.romaji_buf)
                    .hint_text("Romaji")
                    .desired_width(f32::INFINITY),
            );
            if re.changed() {
                repopulate_suggestion_cache = true;
            }
            re.request_focus()
        });
    ui.separator();
    // region: input state change handling
    let segs = segment(&app.romaji_buf);
    let new_len = segs.len();
    if new_len > app.last_segs_len {
        segmentation_count_changed = true;
    }
    app.last_segs_len = new_len;
    if app.selected_segment != app.last_selected_segment {
        repopulate_suggestion_cache = true;
    }
    app.last_selected_segment = app.selected_segment;
    // endregion: input state change handling
    let japanese = conv::to_japanese(&segs, &app.intp, &app.kanji_db);
    'intp_select_ui: {
        let i = app.selected_segment;
        let Some(seg) = segs.get(i) else {
            break 'intp_select_ui;
        };
        ui.horizontal(|ui| {
            intp_button(&mut app.intp, i, ui, "は", "F5", Intp::Hiragana);
            ui.separator();
            intp_button(&mut app.intp, i, ui, "ハ", "F6", Intp::Katakana);
            ui.separator();
            intp_button(&mut app.intp, i, ui, "ha", "F7", Intp::AsIs);
        });
        ui.separator();
        egui::ScrollArea::vertical()
            .max_height(100.0)
            .show(ui, |ui| {
                let hiragana = decompose(seg, &HIRAGANA).to_kana_string();
                let hiragana = hiragana.trim();
                let katakana = decompose(seg, &KATAKANA).to_kana_string();
                let katakana = katakana.trim();
                gen_dict_ui_for_hiragana(ui, &mut app.intp, i, &app.cached_suggestions);
                for pair in crate::radicals::by_name(hiragana) {
                    if ui
                        .button(format!("{} ({} radical)", pair.ch, pair.name))
                        .clicked()
                    {
                        app.intp.insert(i, Intp::Radical(pair));
                        ui.close_menu();
                    }
                }
                ui.separator();
                for (db_idx, kanji) in app.kanji_db.kanji.iter().enumerate() {
                    if (kanji.readings.contains(&hiragana) || kanji.readings.contains(&katakana))
                        && ui
                            .button(format!("{} - {}", kanji.chars[0], kanji.meaning))
                            .clicked()
                    {
                        app.intp.insert(i, Intp::Kanji { db_idx });
                    }
                }
            });
    }
    egui::ScrollArea::vertical()
        .id_source("kana_scroll")
        .show(ui, |ui| {
            let len = segs.len();
            if len != 0 {
                if f5 {
                    app.intp.insert(app.selected_segment, Intp::Hiragana);
                }
                if f6 {
                    app.intp.insert(app.selected_segment, Intp::Katakana);
                }
                if f7 {
                    app.intp.insert(app.selected_segment, Intp::AsIs);
                }
            }
            ui.horizontal_wrapped(|ui| {
                for (i, seg) in segs.iter().enumerate() {
                    let mut text = egui::RichText::new(*seg);
                    if app.selected_segment == i {
                        text = text.color(Color32::WHITE);
                    }
                    if ui
                        .add(egui::Label::new(text).sense(egui::Sense::click()))
                        .clicked()
                    {
                        app.selected_segment = i;
                    }
                }
            });
            ui.label(&japanese);
            if copy_jap_clicked {
                app.clipboard.set_text(&japanese).unwrap()
            }
        });
    if ctrl_enter {
        app.clipboard.set_text(&japanese).unwrap();
        app.romaji_buf.clear();
        app.intp.clear();
        app.hide_requested = true;
    }
    if segmentation_count_changed {
        eprintln!("Segmentation count changed");
        repopulate_suggestion_cache = true;
        // Set selected segment to newly inserted one
        app.selected_segment = new_len - 1;
        // Remove intp info for deleted segments
        for i in app.last_segs_len..new_len {
            app.intp.remove(&i);
        }
    }
    if repopulate_suggestion_cache {
        eprintln!("Suggestion cache repopulate requested");
        app.repopulate_suggestion_cache();
    }
}

fn intp_button(
    intp_map: &mut IntpMap,
    i: usize,
    ui: &mut egui::Ui,
    button_text: &str,
    shortcut_text: &str,
    intp: Intp,
) {
    let mut text = egui::RichText::new(button_text);
    if intp_map
        .get(&i)
        .is_some_and(|a| intp_matches_approx(a, &intp))
    {
        text = text.color(egui::Color32::YELLOW);
    }
    // Hiragana interpretation is kinda the default, indicate this for the hiragana button
    if matches!(intp, Intp::Hiragana) && !intp_map.contains_key(&i) {
        text = text.color(egui::Color32::from_rgb(26, 226, 171));
    }
    if ui
        .add(egui::Button::new(text).shortcut_text(shortcut_text))
        .clicked()
    {
        intp_map.insert(i, intp);
        ui.close_menu();
    }
}

/// Approximate match for intp (can't derive PartialEq)
fn intp_matches_approx(a: &Intp, b: &Intp) -> bool {
    let a_disc = std::mem::discriminant(a);
    let b_disc = std::mem::discriminant(b);
    a_disc == b_disc
}

fn gen_dict_ui_for_hiragana(
    ui: &mut egui::Ui,
    intp: &mut IntpMap,
    i: usize,
    suggestions: &CachedSuggestions,
) {
    for suggestion in suggestions.jmdict.iter() {
        for (ki, kanji_str) in suggestion
            .entry
            .kanji_elements()
            .map(|e| e.text)
            .enumerate()
        {
            let hover_ui = |ui: &mut egui::Ui| {
                ui.set_max_width(400.0);
                dict_en_ui(ui, &suggestion.entry);
            };
            if ui.button(kanji_str).on_hover_ui(hover_ui).clicked() {
                intp.insert(
                    i,
                    Intp::Dictionary {
                        en: suggestion.entry,
                        kanji_idx: ki,
                        root: suggestion.mugo_root.clone(),
                    },
                );
                ui.close_menu();
                return;
            }
        }
    }
}
