use {
    crate::{
        appstate::{AppState, UiState},
        conv::{decompose, HIRAGANA},
    },
    egui_sfml::egui,
};

pub fn dict_ui(ui: &mut egui::Ui, app: &mut AppState) {
    if ui.link("Back").clicked() {
        app.ui_state = UiState::Input;
    }
    ui.columns(2, |cols| {
        dict_list_ui(&mut cols[0], app);
        dict_en_ui(&mut cols[1], app);
    });
}

fn dict_list_ui(ui: &mut egui::Ui, app: &mut AppState) {
    let re =
        ui.add(egui::TextEdit::singleline(&mut app.dict_ui_state.search_buf).hint_text("Filter"));
    if re.changed() {
        app.dict_ui_state.selected = 0;
        let kana = decompose(&app.dict_ui_state.search_buf, &HIRAGANA).to_kana_string();
        app.dict_ui_state.entry_buf = jmdict::entries()
            .filter(|en| en.reading_elements().any(|elem| elem.text.contains(&kana)))
            .collect();
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
                        en.reading_elements().next().unwrap().text,
                    )
                    .clicked()
                {
                    app.dict_ui_state.selected = idx;
                }
            }
        },
    );
}

fn dict_en_ui(ui: &mut egui::Ui, app: &mut AppState) {
    let Some(en) = app.dict_ui_state.entry_buf.get(app.dict_ui_state.selected) else {
        ui.label("<Couldn't get entry>");
        return;
    };
    egui::ScrollArea::vertical()
        .id_source("en_scroll_vert")
        .show(ui, |ui| {
            ui.heading("Kanji elements");
            for elem in en.kanji_elements() {
                ui.label(elem.text);
            }
            ui.heading("Reading elements");
            for elem in en.reading_elements() {
                ui.label(elem.text);
            }
            ui.heading("Senses");
            for sense in en.senses() {
                for gloss in sense.glosses() {
                    ui.label(gloss.text);
                }
                ui.separator();
            }
        });
}

pub struct DictUiState {
    search_buf: String,
    entry_buf: Vec<jmdict::Entry>,
    selected: usize,
}

impl Default for DictUiState {
    fn default() -> Self {
        Self {
            search_buf: Default::default(),
            entry_buf: jmdict::entries().collect(),
            selected: 0,
        }
    }
}
