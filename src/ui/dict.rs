use {
    super::dict_en_ui,
    crate::{
        appstate::{AppState, UiState},
        conv::{decompose, HIRAGANA},
    },
    egui_sfml::egui,
};

pub fn dict_ui(ui: &mut egui::Ui, app: &mut AppState) {
    ui.horizontal(|ui| {
        if ui.link("Back (Esc)").clicked() || ui.input(|inp| inp.key_pressed(egui::Key::Escape)) {
            app.ui_state = UiState::Input;
        }
        ui.separator();
        if ui
            .selectable_label(
                matches!(app.dict_ui_state.lookup_method, LookupMethod::ByKana),
                "By kana",
            )
            .clicked()
        {
            app.dict_ui_state.lookup_method = LookupMethod::ByKana;
        }
        if ui
            .selectable_label(
                matches!(app.dict_ui_state.lookup_method, LookupMethod::ByEnglish),
                "By english",
            )
            .clicked()
        {
            app.dict_ui_state.lookup_method = LookupMethod::ByEnglish;
        }
    });
    ui.columns(2, |cols| {
        dict_list_ui(&mut cols[0], app);
        let Some(en) = app.dict_ui_state.entry_buf.get(app.dict_ui_state.selected) else {
            cols[1].label("<Couldn't get entry>");
            return;
        };
        dict_en_ui(&mut cols[1], en);
    });
    app.dict_ui_state.just_opened = false;
}

fn dict_list_ui(ui: &mut egui::Ui, app: &mut AppState) {
    let re =
        ui.add(egui::TextEdit::singleline(&mut app.dict_ui_state.search_buf).hint_text("Filter"));
    if re.changed() {
        app.dict_ui_state.selected = 0;
        match app.dict_ui_state.lookup_method {
            LookupMethod::ByKana => {
                let kana = decompose(&app.dict_ui_state.search_buf, &HIRAGANA).to_kana_string();
                app.dict_ui_state.entry_buf = jmdict::entries()
                    .filter(|en| en.reading_elements().any(|elem| elem.text.contains(&kana)))
                    .collect()
            }
            LookupMethod::ByEnglish => {
                app.dict_ui_state.entry_buf = jmdict::entries()
                    .filter(|en| {
                        en.senses().any(|sense| {
                            sense
                                .glosses()
                                .any(|gloss| gloss.text.contains(&app.dict_ui_state.search_buf))
                        })
                    })
                    .collect()
            }
        }
    }
    if app.dict_ui_state.just_opened {
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
                if ui
                    .selectable_label(
                        app.dict_ui_state.selected == idx,
                        match app.dict_ui_state.lookup_method {
                            LookupMethod::ByKana => en.reading_elements().next().unwrap().text,
                            LookupMethod::ByEnglish => {
                                en.senses().next().unwrap().glosses().next().unwrap().text
                            }
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
    search_buf: String,
    entry_buf: Vec<jmdict::Entry>,
    selected: usize,
    pub just_opened: bool,
    lookup_method: LookupMethod,
}

enum LookupMethod {
    ByKana,
    ByEnglish,
}

impl Default for DictUiState {
    fn default() -> Self {
        Self {
            search_buf: Default::default(),
            entry_buf: jmdict::entries().collect(),
            selected: 0,
            just_opened: false,
            lookup_method: LookupMethod::ByKana,
        }
    }
}
