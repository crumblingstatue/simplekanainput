mod dict;
pub mod input;
mod kanji_ui;

pub use self::{
    dict::{dict_ui, DictUiState},
    input::input_ui,
    kanji_ui::{kanji_ui, KanjiUiState},
};
use {
    crate::appstate::RootKindExt,
    egui_sfml::egui::{self, text::LayoutJob, TextFormat},
};

fn char_is_hiragana(ch: char) -> bool {
    (0x3040..0x309F).contains(&(ch as u32))
}

fn dict_en_ui(
    ui: &mut egui::Ui,
    en: &jmdict::Entry,
    root: Option<&mugo::Root>,
    mut kanji_idx: Option<&mut usize>,
) {
    egui::ScrollArea::vertical()
        .id_source("en_scroll_vert")
        .show(ui, |ui| {
            let mut steps_str = String::new();
            for elem in en.reading_elements() {
                steps_str.push_str(elem.text);
                steps_str.push_str(", ");
            }
            steps_str.truncate(steps_str.trim_end_matches(", ").len());
            if let Some(root) = root {
                steps_str.push_str(" (");
                steps_str.push_str(mugo_root_kind_label(root.kind));
                steps_str.push_str(") ➡ ");
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
                        mugo::Step::Passive => "passive",
                        mugo::Step::Tai => "たい",
                        mugo::Step::Ba => "ば (conditional)",
                        mugo::Step::Potential => "potential",
                        mugo::Step::Chau => "ちゃう",
                        mugo::Step::Na => "な",
                        mugo::Step::Katta => "かった",
                        mugo::Step::Stem => "stem",
                        mugo::Step::Nu => "ぬ",
                        mugo::Step::Ki => "き (archaic い)",
                        mugo::Step::Nda => "んだ",
                    });
                    if i != root.steps.len() - 1 {
                        steps_str.push('➡');
                    }
                }
            }
            ui.label(
                egui::RichText::new(steps_str)
                    .color(egui::Color32::LIGHT_BLUE)
                    .size(14.0),
            );
            ui.horizontal(|ui| {
                for (i, elem) in en.kanji_elements().enumerate() {
                    ui.spacing_mut().item_spacing = egui::vec2(8.0, 0.0);
                    let mut layout_job = LayoutJob::default();
                    for char in elem.text.chars() {
                        let char_str = char.to_string();
                        if char_is_hiragana(char) {
                            layout_job.append(
                                &char_str,
                                0.0,
                                TextFormat {
                                    font_id: egui::FontId::new(
                                        14.0,
                                        egui::FontFamily::Proportional,
                                    ),
                                    color: egui::Color32::DARK_GRAY,
                                    ..Default::default()
                                },
                            );
                        } else {
                            layout_job.append(
                                &char_str,
                                0.0,
                                TextFormat {
                                    font_id: egui::FontId::new(
                                        18.0,
                                        egui::FontFamily::Proportional,
                                    ),
                                    color: egui::Color32::WHITE,
                                    ..Default::default()
                                },
                            );
                        }
                    }
                    match &mut kanji_idx {
                        Some(idx) => {
                            ui.style_mut().visuals.selection.bg_fill =
                                egui::Color32::from_rgb(27, 7, 29);
                            if ui.selectable_label(i == **idx, layout_job).clicked() {
                                **idx = i;
                            }
                        }
                        None => {
                            ui.label(layout_job);
                        }
                    }
                }
            });
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

                        if let Some(root) = root {
                            // If the root kind equals the jmdict part, we don't need
                            // to show it again.
                            // We're already showing it earlier
                            if root.kind.to_jmdict_part_of_speech() == part {
                                parts_string.push_str("✅ ");
                                continue;
                            }
                        }

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
}

fn mugo_root_kind_label(kind: mugo::RootKind) -> &'static str {
    match kind {
        mugo::RootKind::Ichidan => "ichidan",
        mugo::RootKind::GodanBu => "ぶ",
        mugo::RootKind::GodanMu => "む",
        mugo::RootKind::GodanNu => "ぬ",
        mugo::RootKind::GodanRu => "godan る",
        mugo::RootKind::GodanSu => "す",
        mugo::RootKind::GodanTsu => "つ",
        mugo::RootKind::GodanU => "う",
        mugo::RootKind::GodanGu => "ぐ",
        mugo::RootKind::GodanKu => "く",
        mugo::RootKind::Iku => "行く",
        mugo::RootKind::Kuru => "来る",
        mugo::RootKind::Suru => "する",
        mugo::RootKind::SpecialSuru => "する (special)",
        mugo::RootKind::IAdjective => "い adjective",
        mugo::RootKind::NaAdjective => "な adjective",
    }
}
