mod dict;
mod input;
mod kanji_ui;

pub use self::{
    dict::{dict_ui, DictUiState},
    input::input_ui,
    kanji_ui::{kanji_ui, KanjiUiState},
};
use egui_sfml::egui;

enum DictUiMsg {
    None,
    KanjiClicked(char),
}

fn dict_en_ui(ui: &mut egui::Ui, en: &jmdict::Entry) -> DictUiMsg {
    let mut msg = DictUiMsg::None;
    egui::ScrollArea::vertical()
        .id_source("en_scroll_vert")
        .show(ui, |ui| {
            for elem in en.kanji_elements() {
                ui.horizontal(|ui| {
                    for char in elem.text.chars() {
                        let kanji_str = char.to_string();
                        if ui
                            .add(egui::Label::new(&kanji_str).sense(egui::Sense::click()))
                            .on_hover_ui(|ui| {
                                if let Some(en) = jmdict::entries()
                                    .find(|en| en.kanji_elements().any(|k| k.text == kanji_str))
                                {
                                    dict_en_ui(ui, &en);
                                }
                            })
                            .clicked()
                        {
                            msg = DictUiMsg::KanjiClicked(char);
                        }
                    }
                    ui.label("(");
                    for elem in en.reading_elements() {
                        ui.label(elem.text);
                    }
                    ui.label(")");
                });
            }
            ui.separator();
            for sense in en.senses() {
                ui.horizontal_wrapped(|ui| {
                    let mut gloss_string = String::new();
                    for gloss in sense.glosses() {
                        gloss_string += gloss.text;
                        gloss_string.push_str(", ");
                    }
                    ui.label(egui::RichText::new(gloss_string.trim_end_matches(", ")).size(16.0));
                    let mut parts_string = String::new();
                    for part in sense.parts_of_speech() {
                        parts_string += &part.to_string();
                    }
                    ui.label(
                        egui::RichText::new(parts_string.trim_end_matches(", "))
                            .size(16.0)
                            .italics()
                            .color(egui::Color32::DARK_GRAY),
                    );
                });
            }
        });
    msg
}
