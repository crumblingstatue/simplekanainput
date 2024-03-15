use {
    super::dict_en_ui,
    crate::{
        appstate::{AppState, CachedSuggestions, UiState},
        conv::{self, romaji_to_kana, with_input_span_converted_form, Intp, IntpMap},
        kana::{HIRAGANA, KATAKANA},
        kanji::KanjiDb,
        segment::InputSpan,
    },
    egui_extras::{Size, StripBuilder},
    egui_sfml::egui::{
        self,
        text::{CCursor, CCursorRange},
        Color32, Modifiers,
    },
};

/// Code that does some sanity checks on the application state, and corrects
/// bad state.
///
/// Most of what this fixes is technically bugs, but it's better to have mostly sane behavior
/// even in case of unforeseen bugs, or bugs that haven't been hunted down yet.
fn ensure_ui_sanity(app: &mut AppState) {
    if !app.segments.is_empty() && app.selected_segment >= app.segments.len() {
        segment_sel_nav_left(app);
        eprintln!("App segment selection out of bounds. Corrected.");
    }
}

pub enum InputUiAction {
    SetCursor(usize),
}

pub fn input_ui(ui: &mut egui::Ui, app: &mut AppState) {
    ensure_ui_sanity(app);
    let mut repopulate_suggestion_cache = false;
    let mut copy_jap_clicked = false;
    let (ctrl_enter, f1, f2, f3, f5, f6, esc, tab, shift, alt_left, alt_right) =
        ui.input_mut(|inp| {
            (
                inp.consume_key(Modifiers::CTRL, egui::Key::Enter),
                inp.key_pressed(egui::Key::F1),
                inp.key_pressed(egui::Key::F2),
                inp.key_pressed(egui::Key::F3),
                inp.key_pressed(egui::Key::F5),
                inp.key_pressed(egui::Key::F6),
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
        segment_sel_nav_left(app);
    }
    if alt_right {
        loop {
            app.selected_segment += 1;
            if app.selected_segment >= app.segments.len() {
                app.selected_segment = app.segments.len().saturating_sub(1);
                break;
            }
            if app.segments[app.selected_segment].is_romaji_word() {
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
        // Indicate attribute map status by disabling button if it's empty
        if ui
            .add_enabled(
                !app.intp.is_empty(),
                egui::Button::new("[F3] 🗑 Clear attributes"),
            )
            .clicked()
            || f3
        {
            app.intp.clear();
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("🚪 Quit").clicked() {
                app.quit_requested = true;
            }
            ui.menu_button("☰ Menu", |ui| {
                if ui.button("Kanji dict").clicked() {
                    app.ui_state = UiState::Kanji;
                    ui.close_menu();
                }
                if ui.button("Normalize case").clicked() {
                    app.romaji_buf.make_ascii_lowercase();
                    ui.close_menu();
                }
            });
        });
    });
    ui.separator();
    // Character (not byte) position of the text cursor in the romaji editor
    let mut text_cursor = 0;
    let mut set_textedit_scroll_offset = None;
    let mut scroll_out = egui::ScrollArea::vertical()
        .max_height(120.0)
        .id_source("romaji_scroll")
        .show(ui, |ui| {
            let mut out = egui::TextEdit::multiline(&mut app.romaji_buf)
                .hint_text("Romaji")
                .desired_width(f32::INFINITY)
                .show(ui);
            if out.response.changed() {
                repopulate_suggestion_cache = true;
            }
            if let Some(range) = &mut out.cursor_range {
                text_cursor = range.primary.ccursor.index;
            }
            if let Some(InputUiAction::SetCursor(pos)) = app.input_ui_action.as_ref() {
                out.state
                    .cursor
                    .set_char_range(Some(CCursorRange::one(CCursor::new(*pos))));
                out.state.store(ui.ctx(), out.response.id);
                set_textedit_scroll_offset = Some(app.out_scroll_last_offset);
                app.input_ui_action = None;
            }
            out.response.request_focus()
        });
    if let Some(offset) = set_textedit_scroll_offset {
        scroll_out.state.offset.y = offset;
        scroll_out.state.store(ui.ctx(), scroll_out.id);
    }
    ui.separator();
    // region: input state change handling
    let mut segmentation_count_changed = false;
    let new = crate::segment::segment(&app.romaji_buf);
    crate::detect_edit::detect_edit_update_index_map(&mut app.intp, &app.segments, &new);
    app.segments = new;
    let new_len = app.segments.len();
    if new_len > app.last_segs_len {
        segmentation_count_changed = true;
    }
    app.last_segs_len = new_len;
    if app.selected_segment != app.last_selected_segment {
        repopulate_suggestion_cache = true;
    }
    app.last_selected_segment = app.selected_segment;
    // endregion: input state change handling
    let japanese = conv::to_japanese(&app.romaji_buf, &app.segments, &app.intp, &app.kanji_db);
    StripBuilder::new(ui)
        .size(Size::exact(120.0))
        .size(Size::remainder())
        .vertical(|mut strip| {
            strip.cell(|ui| {
                let scroll_out = egui::ScrollArea::vertical()
                    .id_source("kana_scroll")
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        let len = app.segments.len();
                        if len != 0 {
                            if f5 {
                                app.intp.insert(app.selected_segment, Intp::Hiragana);
                            }
                            if f6 {
                                app.intp.insert(app.selected_segment, Intp::Katakana);
                            }
                        }
                        ui.horizontal_wrapped(|ui| {
                            let spacing = ui.spacing_mut();
                            spacing.item_spacing = egui::vec2(0.0, 0.0);
                            for (i, span) in app.segments.iter().enumerate() {
                                let mut remove_intp = None;
                                with_input_span_converted_form(
                                    span,
                                    i,
                                    &app.romaji_buf,
                                    &app.intp,
                                    &app.kanji_db,
                                    |conv_text| {
                                        let mut text = egui::RichText::new(conv_text);
                                        if span.contains_cursor(text_cursor) {
                                            text = text.color(Color32::LIGHT_BLUE);
                                        }
                                        if app.selected_segment == i {
                                            text = text.color(Color32::YELLOW);
                                        }
                                        let mut re = ui.add(
                                            egui::Label::new(text).sense(egui::Sense::click()),
                                        );
                                        re.context_menu(|ui| {
                                            if ui.button("Edit here").clicked() {
                                                app.input_ui_action = Some(
                                                    InputUiAction::SetCursor(span.cursor_end_pos()),
                                                );
                                                app.selected_segment = i;
                                                ui.close_menu();
                                            }
                                            if ui
                                                .add_enabled(
                                                    app.intp.contains_key(&i),
                                                    egui::Button::new("Clear attribute"),
                                                )
                                                .clicked()
                                            {
                                                remove_intp = Some(i);
                                                ui.close_menu();
                                            }
                                        });
                                        let (InputSpan::Other { start, end }
                                        | InputSpan::RomajiPunct { start, end }
                                        | InputSpan::RomajiWord { start, end }) = *span;
                                        re = re.on_hover_text(&app.romaji_buf[start..end]);
                                        if re.clicked() {
                                            app.selected_segment = i;
                                        }
                                    },
                                );
                                if let Some(idx) = remove_intp {
                                    app.intp.remove(&idx);
                                }
                            }
                        });
                        if copy_jap_clicked {
                            app.clipboard.set_text(&japanese).unwrap()
                        }
                    });
                app.out_scroll_last_offset = scroll_out.state.offset.y;
            });
            strip.strip(|builder| {
                let Some(&InputSpan::RomajiWord { start, end }) =
                    app.segments.get(app.selected_segment)
                else {
                    return;
                };
                let romaji = &app.romaji_buf[start..end];
                suggestion_ui_strip(
                    romaji,
                    app.selected_segment,
                    &mut app.intp,
                    &app.cached_suggestions,
                    &app.kanji_db,
                    builder,
                    sel_changed,
                );
            });
        });
    if ctrl_enter {
        app.clipboard.set_text(&japanese).unwrap();
        app.romaji_buf.clear();
        app.intp.clear();
        app.hide_requested = true;
    }
    if segmentation_count_changed {
        repopulate_suggestion_cache = true;
        // Set selected segment to the nearest romaji segment at the cursor
        let mut any_set = false;
        let mut found_cursor_span = false;
        for (i, span) in app.segments.iter().enumerate().rev() {
            if span.contains_cursor(text_cursor) {
                found_cursor_span = true;
            }
            if found_cursor_span && span.is_romaji_word() {
                app.selected_segment = i;
                any_set = true;
                break;
            }
        }
        if !any_set {
            app.selected_segment = 0;
        }
    }
    if repopulate_suggestion_cache {
        // Also clear the selected suggestion
        app.selected_suggestion = None;
        app.repopulate_suggestion_cache();
    }
}

fn segment_sel_nav_left(app: &mut AppState) {
    loop {
        app.selected_segment = app.selected_segment.saturating_sub(1);
        if app.segments.is_empty() {
            break;
        }
        if app.selected_segment == 0
            || app
                .segments
                .get(app.selected_segment)
                .is_some_and(|seg| seg.is_romaji_word())
        {
            break;
        }
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
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        intp_button(intp, intp_idx, ui, "は", "Hiragana (F5)", Intp::Hiragana);
                        ui.separator();
                        intp_button(intp, intp_idx, ui, "ハ", "Katakana (F6)", Intp::Katakana);
                    });
                    ui.separator();
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
                ui.separator();
                if let Some(Intp::Dictionary {
                    en,
                    root,
                    kanji_idx,
                    ..
                }) = intp.get_mut(&intp_idx)
                {
                    dict_en_ui(ui, en, root.as_ref(), Some(kanji_idx));
                }
            })
        });
}

fn intp_button(
    intp_map: &mut IntpMap,
    i: usize,
    ui: &mut egui::Ui,
    button_text: &str,
    hover_text: &str,
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
        .add(egui::Button::new(text))
        .on_hover_text(hover_text)
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
        let kanji_str = suggestion
            .entry
            .kanji_elements()
            .map(|e| e.text)
            .next()
            .unwrap_or("??? (bug)");
        let hover_ui = |ui: &mut egui::Ui| {
            ui.set_max_width(400.0);
            dict_en_ui(ui, &suggestion.entry, suggestion.mugo_root.as_ref(), None);
        };
        let mut scroll = false;
        let mut selected = false;
        if let Some(Intp::Dictionary { cached_sug_idx, .. }) = intp.get(&intp_idx) {
            if *cached_sug_idx == si {
                selected = true;
                scroll = true;
            }
        }
        let re = ui
            .selectable_label(selected, kanji_str)
            .on_hover_ui(hover_ui);
        if scroll && sel_changed {
            re.scroll_to_me(Some(egui::Align::Center));
        }
        if re.clicked() {
            intp.insert(
                intp_idx,
                Intp::Dictionary {
                    cached_sug_idx: si,
                    en: suggestion.entry,
                    kanji_idx: 0,
                    root: suggestion.mugo_root.clone(),
                },
            );
            ui.close_menu();
            return;
        }
    }
}
