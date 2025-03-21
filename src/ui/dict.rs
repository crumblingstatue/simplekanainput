use {
    super::dict_en_ui_scroll,
    crate::{
        appstate::{AppState, UiState},
        conv::romaji_to_kana,
        egui,
        kana::HIRAGANA,
    },
};

pub fn dict_ui(ui: &mut egui::Ui, app: &mut AppState) {
    let (esc, up_arrow, down_arrow, f2, f3, f4) = ui.input(|inp| {
        (
            inp.key_pressed(egui::Key::Escape),
            inp.key_pressed(egui::Key::ArrowUp),
            inp.key_pressed(egui::Key::ArrowDown),
            inp.key_pressed(egui::Key::F2),
            inp.key_pressed(egui::Key::F3),
            inp.key_pressed(egui::Key::F4),
        )
    });
    if up_arrow {
        app.dict_ui_state.selected = app.dict_ui_state.selected.saturating_sub(1);
    }
    if down_arrow {
        app.dict_ui_state.selected += 1;
    }
    let mut want_focus = false;
    ui.horizontal(|ui| {
        if ui.link("Back (Esc)").clicked() || esc {
            app.ui_state = UiState::Input;
        }
        ui.separator();
        if ui
            .selectable_label(
                matches!(app.dict_ui_state.lookup_method, LookupMethod::Kana),
                "[F2] By kana",
            )
            .clicked()
            || f2
        {
            app.dict_ui_state.lookup_method = LookupMethod::Kana;
            want_focus = true;
        }
        if ui
            .selectable_label(
                matches!(app.dict_ui_state.lookup_method, LookupMethod::Kanji),
                "[F3] By kanji",
            )
            .clicked()
            || f3
        {
            app.dict_ui_state.lookup_method = LookupMethod::Kanji;
            want_focus = true;
        }
        if ui
            .selectable_label(
                matches!(app.dict_ui_state.lookup_method, LookupMethod::English),
                "[F4] By english",
            )
            .clicked()
            || f4
        {
            app.dict_ui_state.lookup_method = LookupMethod::English;
            want_focus = true;
        }
        crate::ui::show_menu_button(app, ui);
    });
    ui.columns(2, |cols| {
        dict_list_ui(&mut cols[0], app);
        let Some(en) = app.dict_ui_state.entry_buf.get(app.dict_ui_state.selected) else {
            cols[1].label("<Couldn't get entry>");
            return;
        };
        dict_en_ui_scroll(&mut cols[1], en, None, None);
    });
    app.dict_ui_state.focus_textinput = want_focus;
}

#[derive(Debug, PartialEq)]
struct KanjiQuery<'s> {
    contains: &'s str,
    starts_with: Option<char>,
    n_chars: Option<u8>,
}

impl<'s> KanjiQuery<'s> {
    fn from_str(src: &'s str) -> Self {
        let mut contains = src;
        let mut begins_with = None;
        let mut n_chars = None;
        if let Some(text) = src.strip_prefix('^') {
            if let Some((idx, ch)) = text.char_indices().next() {
                begins_with = Some(ch);
                contains = &text[idx + ch.len_utf8()..];
            }
        }
        if let Some(open_idx) = contains.find('(')
            && let Some(close_idx) = contains[open_idx..].find(')')
            && let Some(contents) = contains.get(open_idx + 1..open_idx + close_idx)
        {
            contains = &contains[..open_idx];
            if let Ok(num) = contents.parse() {
                n_chars = Some(num);
            }
        }
        Self {
            contains,
            starts_with: begins_with,
            n_chars,
        }
    }
}

#[test]
fn test_kanji_query_from_str() {
    assert_eq!(
        KanjiQuery::from_str("べる(3)"),
        KanjiQuery {
            contains: "べる",
            starts_with: None,
            n_chars: Some(3)
        }
    );
    assert_eq!(
        KanjiQuery::from_str("^変化(2)"),
        KanjiQuery {
            contains: "化",
            starts_with: Some('変'),
            n_chars: Some(2)
        }
    );
}

fn dict_list_ui(ui: &mut egui::Ui, app: &mut AppState) {
    let search_buf = match app.dict_ui_state.lookup_method {
        LookupMethod::Kana => &mut app.dict_ui_state.kana_search_buf,
        LookupMethod::English => &mut app.dict_ui_state.english_search_buf,
        LookupMethod::Kanji => &mut app.dict_ui_state.kanji_search_buf,
    };
    let mut kana_converted = false;
    let re = ui
        .horizontal(|ui| {
            if ui
                .button("🈂")
                .on_hover_text("Turn input into hiragana (ctrl+space)")
                .clicked()
                || ui.input(|inp| inp.key_pressed(egui::Key::Space) && inp.modifiers.ctrl)
            {
                let out = crate::conv::romaji_to_kana(search_buf, &HIRAGANA);
                if !out.is_empty() {
                    *search_buf = out;
                }
                kana_converted = true;
            }
            ui.add(egui::TextEdit::singleline(search_buf).hint_text("Filter"))
        })
        .inner;
    if re.changed() || kana_converted || app.dict_ui_state.focus_textinput {
        app.dict_ui_state.selected = 0;
        match app.dict_ui_state.lookup_method {
            LookupMethod::Kana => {
                let kana = romaji_to_kana(&app.dict_ui_state.kana_search_buf, &HIRAGANA);
                app.dict_ui_state.entry_buf = jmdict::entries()
                    .filter(|en| en.reading_elements().any(|elem| elem.text.contains(&kana)))
                    .collect();
            }
            LookupMethod::English => {
                app.dict_ui_state.entry_buf = jmdict::entries()
                    .filter(|en| {
                        en.senses().any(|sense| {
                            sense.glosses().any(|gloss| {
                                gloss.text.contains(&app.dict_ui_state.english_search_buf)
                            })
                        })
                    })
                    .collect();
                app.dict_ui_state.entry_buf.sort_by_key(|en| {
                    strsim::levenshtein(
                        &app.dict_ui_state.english_search_buf,
                        en.senses().next().unwrap().glosses().next().unwrap().text,
                    )
                });
            }
            LookupMethod::Kanji => {
                let query = KanjiQuery::from_str(&app.dict_ui_state.kanji_search_buf);
                app.dict_ui_state.entry_buf = jmdict::entries()
                    .filter(|en| {
                        en.kanji_elements().any(|elem| {
                            elem.text.contains(query.contains)
                                && query.starts_with.is_none_or(|ch| elem.text.starts_with(ch))
                                && query
                                    .n_chars
                                    .is_none_or(|n| elem.text.chars().count() == usize::from(n))
                        })
                    })
                    .collect();
            }
        }
    }
    if app.dict_ui_state.focus_textinput {
        re.request_focus();
    }
    egui::ScrollArea::vertical().show_rows(
        ui,
        24.0,
        app.dict_ui_state.entry_buf.len(),
        |ui, range| {
            ui.set_min_width(200.0);
            let start = range.start;
            for (i, en) in app.dict_ui_state.entry_buf[range].iter().enumerate() {
                let idx = start + i;
                let s;
                if ui
                    .selectable_label(
                        app.dict_ui_state.selected == idx,
                        match app.dict_ui_state.lookup_method {
                            LookupMethod::Kana => match en.kanji_elements().next() {
                                Some(kanji) => {
                                    s = format!(
                                        "{} ({})",
                                        kanji.text,
                                        en.reading_elements().next().unwrap().text
                                    );
                                    &s
                                }
                                None => en.reading_elements().next().unwrap().text,
                            },
                            LookupMethod::English => match en.kanji_elements().next() {
                                Some(kanji) => {
                                    s = format!(
                                        "{} ({})",
                                        en.senses().next().unwrap().glosses().next().unwrap().text,
                                        kanji.text,
                                    );
                                    &s
                                }
                                None => {
                                    s = format!(
                                        "{} ({})",
                                        en.senses().next().unwrap().glosses().next().unwrap().text,
                                        en.reading_elements().next().unwrap().text
                                    );
                                    &s
                                }
                            },
                            LookupMethod::Kanji => match en.kanji_elements().next() {
                                Some(elem) => elem.text,
                                None => en.reading_elements().next().unwrap().text,
                            },
                        },
                    )
                    .clicked()
                {
                    app.dict_ui_state.selected = idx;
                }
            }
        },
    );
}

pub struct DictUiState {
    kana_search_buf: String,
    kanji_search_buf: String,
    english_search_buf: String,
    entry_buf: Vec<jmdict::Entry>,
    selected: usize,
    pub focus_textinput: bool,
    lookup_method: LookupMethod,
}

enum LookupMethod {
    Kana,
    English,
    Kanji,
}

impl Default for DictUiState {
    fn default() -> Self {
        Self {
            kana_search_buf: String::new(),
            kanji_search_buf: String::new(),
            english_search_buf: String::new(),
            entry_buf: jmdict::entries().collect(),
            selected: 0,
            focus_textinput: false,
            lookup_method: LookupMethod::Kana,
        }
    }
}
