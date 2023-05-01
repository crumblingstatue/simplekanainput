mod dict;
mod input;

pub use self::{
    dict::{dict_ui, DictUiState},
    input::input_ui,
};
use egui_sfml::egui;

fn dict_en_ui(ui: &mut egui::Ui, en: &jmdict::Entry) {
    egui::ScrollArea::vertical()
        .id_source("en_scroll_vert")
        .show(ui, |ui| {
            ui.label(egui::RichText::new("Kanji").size(12.0));
            for elem in en.kanji_elements() {
                ui.label(elem.text);
            }
            ui.separator();
            ui.label(egui::RichText::new("Reading").size(12.0));
            for elem in en.reading_elements() {
                ui.label(elem.text);
            }
            ui.separator();
            ui.label(egui::RichText::new("Senses").size(12.0));
            for sense in en.senses() {
                ui.horizontal_wrapped(|ui| {
                    let mut begin = true;
                    for gloss in sense.glosses() {
                        if !begin {
                            ui.separator();
                        }
                        begin = false;
                        ui.label(gloss.text);
                    }
                    ui.end_row();
                    begin = true;
                    for part in sense.parts_of_speech() {
                        if !begin {
                            ui.separator();
                        }
                        begin = false;
                        ui.label(
                            egui::RichText::new(part.to_string())
                                .size(12.0)
                                .color(egui::Color32::DARK_GRAY),
                        );
                    }
                });
            }
        });
}
