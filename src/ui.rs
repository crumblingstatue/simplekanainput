mod dict;
pub mod input;
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

fn dict_en_ui(ui: &mut egui::Ui, en: &jmdict::Entry, root: Option<&mugo::Root>) -> DictUiMsg {
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
                                    dict_en_ui(ui, &en, root);
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
                    if let Some(root) = root {
                        ui.separator();
                        let mut steps_str = String::new();
                        for (i, step) in root.steps.iter().enumerate() {
                            steps_str.push_str(match step {
                                mugo::Step::Te => "て",
                                mugo::Step::Nai => "ない",
                                mugo::Step::Naide => "ないで",
                                mugo::Step::Nakatta => "なかった",
                                mugo::Step::Ta => "た",
                                mugo::Step::Volitional => "volitional",
                                mugo::Step::AdverbialKu => "く (adverb)",
                                mugo::Step::Imperative => "imperative",
                                mugo::Step::Masu => "ます",
                                mugo::Step::Masen => "ません",
                                mugo::Step::Invitational => "invitational",
                                mugo::Step::Continuous => "ている",
                                mugo::Step::ContRuAbbrev => "てる",
                                mugo::Step::Zu => "ず",
                                mugo::Step::Ka => "か",
                                mugo::Step::Tari => "たり",
                                mugo::Step::Tara => "たら",
                                mugo::Step::Nasai => "なさい",
                                mugo::Step::Nagara => "ながら",
                                mugo::Step::Causative => "causative",
                                mugo::Step::Tai => "たい",
                                mugo::Step::Ba => "ば (conditional)",
                                mugo::Step::Potential => "potential",
                                mugo::Step::Chau => "ちゃう",
                                mugo::Step::Na => "な",
                                mugo::Step::Katta => "かった",
                                mugo::Step::Stem => "stem",
                                mugo::Step::Nu => "ぬ",
                            });
                            if i != root.steps.len() - 1 {
                                steps_str.push('➡');
                            }
                        }
                        ui.label(
                            egui::RichText::new(steps_str)
                                .color(egui::Color32::LIGHT_BLUE)
                                .size(14.0),
                        );
                    }
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
                        use jmdict::PartOfSpeech as P;
                        let str = match part {
                            P::Adjective => "adjective",
                            P::CommonNoun => "noun",
                            P::AdjectivalNoun => "な adjective",
                            P::Expression => "expression",
                            P::NoAdjective => "の adjective",
                            P::IchidanVerb => "ichidan verb",
                            P::GodanBuVerb => "ぶ verb",
                            P::GodanGuVerb => "ぐ verb",
                            P::GodanKuVerb => "く verb",
                            P::GodanIkuVerb => "行く verb",
                            P::GodanMuVerb => "む verb",
                            P::GodanNuVerb => "ぬ verb",
                            P::GodanUVerb => "う verb",
                            P::GodanSuVerb => "す verb",
                            P::GodanRuVerb => "godan る verb",
                            P::SuruVerb => "する verb",
                            P::IntransitiveVerb => "intransitive",
                            P::TransitiveVerb => "transitive",
                            P::Adverb => "adverb",
                            P::Pronoun => "pronoun",
                            P::Suffix => "suffix",
                            P::Interjection => "interjection",
                            _ => jmdict::Enum::constant_name(&part),
                        };
                        parts_string.push_str(str);
                        parts_string.push_str(", ");
                    }
                    ui.label(
                        egui::RichText::new(parts_string.trim_end_matches(", "))
                            .size(12.0)
                            .color(egui::Color32::DARK_GRAY),
                    );
                });
            }
        });
    msg
}
