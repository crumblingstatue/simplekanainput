use {
    super::dict_en_ui,
    crate::{
        appstate::{AppState, UiState},
        conv::{self, decompose, Intp, HIRAGANA},
    },
    egui_sfml::egui::{self, Modifiers},
};

pub fn input_ui(ui: &mut egui::Ui, app: &mut AppState) {
    let mut copy_jap_clicked = false;
    let ctrl_enter = ui.input_mut(|inp| inp.consume_key(Modifiers::CTRL, egui::Key::Enter));
    ui.horizontal(|ui| {
        if ui.button("Copy japanese").clicked() {
            copy_jap_clicked = true;
        }
        if ui.button("Clear attribs (debug)").clicked() {
            app.intp.clear();
        }
        if ui.button("Dictionary (F1)").clicked() || ui.input(|inp| inp.key_pressed(egui::Key::F1))
        {
            app.ui_state = UiState::Dict;
            app.dict_ui_state.just_opened = true;
        }
    });
    ui.separator();
    egui::ScrollArea::vertical()
        .max_height(app.half_dims.h.into())
        .id_source("romaji_scroll")
        .show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut app.romaji_buf)
                    .hint_text("Romaji")
                    .desired_width(f32::INFINITY),
            )
            .request_focus();
        });
    ui.separator();
    egui::ScrollArea::vertical()
        .id_source("kana_scroll")
        .show(ui, |ui| {
            let segs = conv::segment(&app.romaji_buf);
            ui.horizontal_wrapped(|ui| {
                for (i, seg) in segs.iter().enumerate() {
                    ui.add(egui::Label::new(seg.label_string()).sense(egui::Sense::click()))
                        .context_menu(|ui| {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                if ui.button("Hiragana").clicked() {
                                    app.intp.insert(i, Intp::Hiragana);
                                    ui.close_menu();
                                }
                                if ui.button("Katakana").clicked() {
                                    app.intp.insert(i, Intp::Katakana);
                                    ui.close_menu();
                                }
                                let kana = decompose(seg.dict_root(), &HIRAGANA).to_kana_string();
                                let kana = kana.trim();
                                if ui.button("as-is (romaji)").clicked() {
                                    app.intp.insert(i, Intp::AsIs);
                                    ui.close_menu();
                                }
                                ui.separator();
                                for e in jmdict::entries() {
                                    if e.reading_elements().any(|e| e.text == kana) {
                                        for kanji_str in e.kanji_elements().map(|e| e.text) {
                                            let hover_ui = |ui: &mut egui::Ui| {
                                                ui.set_max_width(400.0);
                                                dict_en_ui(ui, &e);
                                            };
                                            if ui.button(kanji_str).on_hover_ui(hover_ui).clicked()
                                            {
                                                match seg {
                                                    conv::Segment::Simple(_) => {
                                                        let root = kanji_str.to_owned();
                                                        app.intp.insert(i, Intp::String(root));
                                                    }
                                                    conv::Segment::DictAndExtra {
                                                        dict: _,
                                                        extra,
                                                        cutoff,
                                                    } => {
                                                        let mut s = kanji_str.to_owned();
                                                        for _ in 0..*cutoff {
                                                            s.pop();
                                                        }
                                                        s.push_str(
                                                            &decompose(extra, &HIRAGANA)
                                                                .to_kana_string(),
                                                        );
                                                        app.intp.insert(i, Intp::String(s));
                                                    }
                                                }
                                                ui.close_menu();
                                            }
                                        }
                                    }
                                }
                            });
                        });
                }
            });
            let japanese = conv::to_japanese(&segs, &app.intp);
            ui.label(&japanese);
            if copy_jap_clicked {
                app.clipboard.set_text(&japanese).unwrap()
            }
            if ctrl_enter {
                app.clipboard.set_text(&japanese).unwrap();
                app.quit_requested = true;
            }
        });
}
