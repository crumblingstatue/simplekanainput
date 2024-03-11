use {
    super::dict_en_ui,
    crate::{
        appstate::{AppState, UiState},
        conv::{self, decompose, Intp, IntpMap},
        kana::{HIRAGANA, KATAKANA},
        segment::segment,
    },
    egui_sfml::egui::{self, Color32, Modifiers},
    mugo::RootKind,
    std::borrow::Cow,
};

enum Root<'a> {
    Bare(&'a str),
    Conj(mugo::Root),
}

impl<'a> Root<'a> {
    fn dict_text(&self) -> Cow<str> {
        match self {
            Root::Bare(s) => Cow::Borrowed(s),
            Root::Conj(root) => Cow::Owned(root.dict()),
        }
    }

    fn matches(&self, e: jmdict::Entry) -> bool {
        match self {
            Root::Bare(_) => self.reading_matches(e),
            Root::Conj(root) => {
                root_kind_matches(&root.kind, e.senses()) && self.reading_matches(e)
            }
        }
    }

    fn reading_matches(&self, e: jmdict::Entry) -> bool {
        e.reading_elements().any(|e| e.text == self.dict_text())
    }
}

fn root_kind_matches(kind: &mugo::RootKind, mut senses: jmdict::Senses) -> bool {
    senses.any(|sense| {
        sense
            .parts_of_speech()
            .any(|part| part == kind.to_jmdict_part_of_speech())
    })
}

trait RootKindExt {
    fn to_jmdict_part_of_speech(&self) -> jmdict::PartOfSpeech;
}

impl RootKindExt for RootKind {
    fn to_jmdict_part_of_speech(&self) -> jmdict::PartOfSpeech {
        use jmdict::PartOfSpeech as Part;
        match self {
            RootKind::Ichidan => Part::IchidanVerb,
            RootKind::GodanBu => Part::GodanBuVerb,
            RootKind::GodanMu => Part::GodanMuVerb,
            RootKind::GodanNu => Part::GodanNuVerb,
            RootKind::GodanRu => Part::GodanRuVerb,
            RootKind::GodanSu => Part::GodanSuVerb,
            RootKind::GodanTsu => Part::GodanTsuVerb,
            RootKind::GodanU => Part::GodanUVerb,
            RootKind::GodanGu => Part::GodanGuVerb,
            RootKind::GodanKu => Part::GodanKuVerb,
            RootKind::IAdjective => Part::Adjective,
            RootKind::Iku => Part::GodanIkuVerb,
            RootKind::Kuru => Part::KuruVerb,
            RootKind::NaAdjective => Part::AdjectivalNoun,
            RootKind::Suru => Part::SuruVerb,
            RootKind::SpecialSuru => Part::SpecialSuruVerb,
        }
    }
}

pub fn input_ui(ui: &mut egui::Ui, app: &mut AppState) {
    let mut copy_jap_clicked = false;
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
            ui.add(
                egui::TextEdit::multiline(&mut app.romaji_buf)
                    .hint_text("Romaji")
                    .desired_width(f32::INFINITY),
            )
            .request_focus();
        });
    ui.separator();
    let segs = segment(&app.romaji_buf);
    // region: Segmentation change handling
    let new_len = segs.len();
    if new_len > app.last_segs_len {
        // Set selected segment to newly inserted one
        app.selected_segment = new_len - 1;
        // Remove intp info for deleted segments
        for i in app.last_segs_len..new_len {
            app.intp.remove(&i);
        }
    }
    // endregion: Segmentation change handling
    app.last_segs_len = new_len;
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
                gen_dict_ui_for_hiragana(Root::Bare(hiragana), ui, &mut app.intp, i);
                for root in mugo::deconjugate(hiragana) {
                    gen_dict_ui_for_hiragana(Root::Conj(root), ui, &mut app.intp, i);
                }
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

fn gen_dict_ui_for_hiragana(root: Root, ui: &mut egui::Ui, intp: &mut IntpMap, i: usize) {
    for e in jmdict::entries() {
        if root.matches(e) {
            for (ki, kanji_str) in e.kanji_elements().map(|e| e.text).enumerate() {
                let hover_ui = |ui: &mut egui::Ui| {
                    ui.set_max_width(400.0);
                    dict_en_ui(ui, &e);
                };
                if ui.button(kanji_str).on_hover_ui(hover_ui).clicked() {
                    intp.insert(
                        i,
                        Intp::Dictionary {
                            en: e,
                            kanji_idx: ki,
                            root: match root {
                                Root::Bare(_) => None,
                                Root::Conj(root) => Some(root),
                            },
                        },
                    );
                    ui.close_menu();
                    return;
                }
            }
        }
    }
}
