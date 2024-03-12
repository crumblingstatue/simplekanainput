use {
    super::dict_en_ui,
    crate::{
        appstate::{AppState, CachedSuggestions, UiState},
        conv::{self, romaji_to_kana, Intp, IntpMap},
        kana::{HIRAGANA, KATAKANA},
        kanji::KanjiDb,
        segment::SegmentKind,
    },
    egui_extras::{Size, StripBuilder},
    egui_sfml::egui::{self, Color32, Modifiers},
};

pub fn input_ui(ui: &mut egui::Ui, app: &mut AppState) {
    let mut repopulate_suggestion_cache = false;
    let mut copy_jap_clicked = false;
    let mut segmentation_count_changed = None;
    let (ctrl_enter, f1, f2, f3, f5, f6, f7, esc, tab, shift, alt_left, alt_right) =
        ui.input_mut(|inp| {
            (
                inp.consume_key(Modifiers::CTRL, egui::Key::Enter),
                inp.key_pressed(egui::Key::F1),
                inp.key_pressed(egui::Key::F2),
                inp.key_pressed(egui::Key::F3),
                inp.key_pressed(egui::Key::F5),
                inp.key_pressed(egui::Key::F6),
                inp.key_pressed(egui::Key::F7),
                inp.key_pressed(egui::Key::Escape),
                inp.consume_key(Modifiers::NONE, egui::Key::Tab),
                inp.modifiers.shift,
                inp.consume_key(Modifiers::ALT, egui::Key::ArrowLeft),
                inp.consume_key(Modifiers::ALT, egui::Key::ArrowRight),
            )
        });
    if esc {
        app.hide_requested = true;
    }
    // Navigate to previous/next romaji segment to select
    if alt_left {
        loop {
            app.selected_segment = app.selected_segment.saturating_sub(1);
            if app.segments.is_empty() {
                break;
            }
            if app.selected_segment == 0
                || app
                    .segments
                    .get(app.selected_segment)
                    .is_some_and(|seg| seg.kind == SegmentKind::Romaji)
            {
                break;
            }
        }
    }
    if alt_right {
        loop {
            app.selected_segment += 1;
            if app.selected_segment >= app.segments.len() {
                app.selected_segment = app.segments.len().saturating_sub(1);
                break;
            }
            if app.segments[app.selected_segment].kind == SegmentKind::Romaji {
                break;
            }
        }
    }
    // Propagates to the intp selection ui to scroll to the entry in case selection was changed
    let mut sel_changed = false;
    if tab {
        'tabhandler: {
            let selected_sug = match &mut app.selected_suggestion {
                Some(sug) => {
                    if shift {
                        if *sug == 0 {
                            app.selected_suggestion = None;
                            app.intp.remove(&app.selected_segment);
                            break 'tabhandler;
                        } else {
                            *sug -= 1;
                            sel_changed = true;
                        }
                    } else if (*sug + 1) < app.cached_suggestions.jmdict.len() {
                        *sug += 1;
                        sel_changed = true;
                    }
                    *sug
                }
                None => {
                    app.selected_suggestion = Some(0);
                    0
                }
            };
            // Accept first suggestion if tab is pressed
            if let Some(sug) = app.cached_suggestions.jmdict.get(selected_sug) {
                app.intp.insert(
                    app.selected_segment,
                    Intp::Dictionary {
                        cached_sug_idx: selected_sug,
                        en: sug.entry,
                        kanji_idx: 0,
                        root: sug.mugo_root.clone(),
                    },
                );
            }
        }
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
    app.segments = crate::segment::segment(&app.romaji_buf);
    let new_len = app.segments.len();
    if new_len > app.last_segs_len {
        segmentation_count_changed = Some(app.last_segs_len);
    }
    app.last_segs_len = new_len;
    if app.selected_segment != app.last_selected_segment {
        repopulate_suggestion_cache = true;
    }
    app.last_selected_segment = app.selected_segment;
    // endregion: input state change handling
    let japanese = conv::to_japanese(&app.romaji_buf, &app.segments, &app.intp, &app.kanji_db);
    'intp_select_ui: {
        let i = app.selected_segment;
        let Some(span) = app.segments.get(i) else {
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
        StripBuilder::new(ui)
            .size(Size::exact(120.0))
            .size(Size::remainder())
            .vertical(|mut strip| {
                strip.strip(|builder| {
                    let seg = span.index(&app.romaji_buf);
                    suggestion_ui_strip(
                        seg,
                        i,
                        &mut app.intp,
                        &app.cached_suggestions,
                        &app.kanji_db,
                        builder,
                        sel_changed,
                    );
                });
                strip.cell(|ui| {
                    ui.separator();
                    egui::ScrollArea::vertical()
                        .id_source("kana_scroll")
                        .show(ui, |ui| {
                            let len = app.segments.len();
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
                                for (i, span) in app.segments.iter().enumerate() {
                                    let mut text = egui::RichText::new(span.index(&app.romaji_buf));
                                    if app.selected_segment == i {
                                        text = text.color(Color32::WHITE);
                                    }
                                    if span.kind == SegmentKind::Romaji
                                        && ui
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
                })
            });
    }
    if ctrl_enter {
        app.clipboard.set_text(&japanese).unwrap();
        app.romaji_buf.clear();
        app.intp.clear();
        app.hide_requested = true;
    }
    if let Some(old) = segmentation_count_changed {
        repopulate_suggestion_cache = true;
        // Set selected segment to the last romaji segment or 0
        let mut any_set = false;
        for (i, span) in app.segments.iter().enumerate().rev() {
            if span.kind == SegmentKind::Romaji {
                app.selected_segment = i;
                any_set = true;
                break;
            }
        }
        if !any_set {
            app.selected_segment = 0;
        }
        // Remove intp info for deleted segments
        for i in old..new_len {
            app.intp.remove(&i);
        }
    }
    if repopulate_suggestion_cache {
        // Also clear the selected suggestion
        app.selected_suggestion = None;
        app.repopulate_suggestion_cache();
    }
}

fn suggestion_ui_strip(
    seg: &str,
    intp_idx: usize,
    intp: &mut IntpMap,
    cached_suggestions: &CachedSuggestions,
    kanji_db: &KanjiDb,
    strip_builder: StripBuilder,
    sel_changed: bool,
) {
    strip_builder
        .clip(true)
        .size(Size::exact(100.0))
        .size(Size::remainder().at_least(100.0))
        .horizontal(|mut strip| {
            strip.cell(|ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let hiragana = romaji_to_kana(seg, &HIRAGANA);
                    let hiragana = hiragana.trim();
                    let katakana = romaji_to_kana(seg, &KATAKANA);
                    let katakana = katakana.trim();
                    gen_dict_ui_for_hiragana(ui, intp, intp_idx, cached_suggestions, sel_changed);
                    for pair in crate::radicals::by_name(hiragana) {
                        if ui
                            .button(format!("{} ({} radical)", pair.ch, pair.name))
                            .clicked()
                        {
                            intp.insert(intp_idx, Intp::Radical(pair));
                            ui.close_menu();
                        }
                    }
                    ui.separator();
                    for (db_idx, kanji) in kanji_db.kanji.iter().enumerate() {
                        if (kanji.readings.contains(&hiragana)
                            || kanji.readings.contains(&katakana))
                            && ui
                                .button(format!("{} - {}", kanji.chars[0], kanji.meaning))
                                .clicked()
                        {
                            intp.insert(intp_idx, Intp::Kanji { db_idx });
                        }
                    }
                });
            });
            strip.cell(|ui| {
                if let Some(Intp::Dictionary { en, root, .. }) = intp.get(&intp_idx) {
                    dict_en_ui(ui, en, root.as_ref());
                }
            })
        });
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
    intp_idx: usize,
    suggestions: &CachedSuggestions,
    sel_changed: bool,
) {
    for (si, suggestion) in suggestions.jmdict.iter().enumerate() {
        // Same entry, different kanji goes into horizontal layout
        ui.horizontal(|ui| {
            for (ki, kanji_str) in suggestion
                .entry
                .kanji_elements()
                .map(|e| e.text)
                .enumerate()
            {
                let hover_ui = |ui: &mut egui::Ui| {
                    ui.set_max_width(400.0);
                    dict_en_ui(ui, &suggestion.entry, suggestion.mugo_root.as_ref());
                };
                let mut text = egui::RichText::new(kanji_str);
                let mut scroll = false;
                if let Some(Intp::Dictionary { cached_sug_idx, .. }) = intp.get(&intp_idx) {
                    if *cached_sug_idx == si {
                        text = text.color(egui::Color32::YELLOW);
                        scroll = true;
                    }
                }
                let re = ui.button(text).on_hover_ui(hover_ui);
                if scroll && sel_changed {
                    re.scroll_to_me(Some(egui::Align::Center));
                }
                if re.clicked() {
                    intp.insert(
                        intp_idx,
                        Intp::Dictionary {
                            cached_sug_idx: si,
                            en: suggestion.entry,
                            kanji_idx: ki,
                            root: suggestion.mugo_root.clone(),
                        },
                    );
                    ui.close_menu();
                    return;
                }
            }
        });
    }
}
