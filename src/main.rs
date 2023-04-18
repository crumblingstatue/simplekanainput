#![feature(iterator_try_collect)]

mod conv;

use std::time::Duration;

use arboard::Clipboard;
use conv::{Intp, IntpMap};
use egui_sfml::{
    egui::{self, FontData, FontFamily, Modifiers},
    sfml::{
        graphics::RenderWindow,
        system::Vector2,
        window::{ContextSettings, Event, Style, VideoMode},
    },
    SfEgui,
};

use crate::conv::{decompose, HIRAGANA};

struct WinDims {
    w: u16,
    h: u16,
}

impl WinDims {
    fn to_sf_video_mode(&self) -> VideoMode {
        VideoMode {
            width: self.w.into(),
            height: self.h.into(),
            bits_per_pixel: 32,
        }
    }
    fn half(&self) -> Self {
        Self {
            w: self.w / 2,
            h: self.h / 2,
        }
    }
}

const WIN_DIMS: WinDims = WinDims { w: 640, h: 360 };

fn main() {
    std::panic::set_hook(Box::new(|info| {
        rfd::MessageDialog::new()
            .set_title("Panic")
            .set_description(&info.to_string())
            .show();
    }));
    let mut rw = RenderWindow::new(
        WIN_DIMS.to_sf_video_mode(),
        "Simple Kana Input",
        Style::DEFAULT,
        &ContextSettings::default(),
    );
    rw.set_framerate_limit(60);
    let half_dims = WIN_DIMS.half();
    rw.set_position(Vector2::new(
        1920 / 2 - half_dims.w as i32,
        1080 / 2 - half_dims.h as i32,
    ));
    let mut sf_egui = SfEgui::new(&rw);
    let mut font_defs = egui::FontDefinitions::default();
    font_defs.font_data.insert(
        "ipag".to_owned(),
        FontData::from_static(include_bytes!("../ipag.ttf")),
    );
    font_defs
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "ipag".to_owned());
    sf_egui.context().set_fonts(font_defs);
    let mut style = egui::Style::default();
    for (_text_style, font_id) in style.text_styles.iter_mut() {
        font_id.size = 20.0; // whatever size you want here
    }
    sf_egui.context().set_style(style);
    let mut romaji_buf = String::new();
    let mut clipboard = Clipboard::new().unwrap();
    let mut intp = IntpMap::default();

    while rw.is_open() {
        while let Some(ev) = rw.poll_event() {
            sf_egui.add_event(&ev);

            if let Event::Closed = ev {
                rw.close();
            }
        }
        sf_egui
            .do_frame(|ctx| {
                let mut copy_jap_clicked = false;
                egui::CentralPanel::default().show(ctx, |ui| {
                    let ctrl_enter =
                        ui.input_mut(|inp| inp.consume_key(Modifiers::CTRL, egui::Key::Enter));
                    ui.horizontal(|ui| {
                        if ui.button("Copy japanese").clicked() {
                            copy_jap_clicked = true;
                        }
                        if ui.button("Clear attribs (debug)").clicked() {
                            intp.clear();
                        }
                    });
                    ui.separator();
                    egui::ScrollArea::vertical()
                        .max_height(half_dims.h.into())
                        .id_source("romaji_scroll")
                        .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut romaji_buf)
                                    .hint_text("Romaji")
                                    .desired_width(f32::INFINITY),
                            )
                            .request_focus();
                        });
                    ui.separator();
                    egui::ScrollArea::vertical()
                        .id_source("kana_scroll")
                        .show(ui, |ui| {
                            let segs = conv::segment(&romaji_buf);
                            ui.horizontal_wrapped(|ui| {
                                for (i, &seg) in segs.iter().enumerate() {
                                    ui.add(
                                        egui::Label::new(seg.trim()).sense(egui::Sense::click()),
                                    )
                                    .context_menu(|ui| {
                                        egui::ScrollArea::vertical().show(ui, |ui| {
                                            if ui.button("Hiragana").clicked() {
                                                intp.insert(i, Intp::Hiragana);
                                                ui.close_menu();
                                            }
                                            if ui.button("Katakana").clicked() {
                                                intp.insert(i, Intp::Katakana);
                                                ui.close_menu();
                                            }
                                            let kana = decompose(seg, &HIRAGANA).to_kana_string();
                                            let kana = kana.trim();
                                            if ui.button("as-is (romaji)").clicked() {
                                                intp.insert(i, Intp::AsIs);
                                                ui.close_menu();
                                            }
                                            ui.separator();
                                            for e in jmdict::entries() {
                                                if e.reading_elements().any(|e| e.text == kana) {
                                                    for kanji_str in
                                                        e.kanji_elements().map(|e| e.text)
                                                    {
                                                        if ui
                                                            .button(kanji_str)
                                                            .on_hover_text(hover_string(e))
                                                            .clicked()
                                                        {
                                                            intp.insert(
                                                                i,
                                                                Intp::String(kanji_str.to_owned()),
                                                            );
                                                            ui.close_menu();
                                                        }
                                                    }
                                                }
                                            }
                                        });
                                    });
                                }
                            });
                            let japanese = conv::to_japanese(&segs, &intp);
                            ui.label(&japanese);
                            if copy_jap_clicked {
                                clipboard.set_text(&japanese).unwrap()
                            }
                            if ctrl_enter {
                                clipboard.set_text(&japanese).unwrap();
                                rw.close();
                            }
                        });
                });
            })
            .unwrap();
        sf_egui.draw(&mut rw, None);
        rw.display();
    }
    // Wait for clipboard to synchronize with manager
    std::thread::sleep(Duration::from_secs(1));
}

fn hover_string(e: jmdict::Entry) -> String {
    let mut out = String::new();
    for (tr_i, sense) in e.senses().enumerate() {
        out.push_str(&format!("{tr_i}: "));
        for gloss in sense.glosses() {
            out.push_str(gloss.text);
            out.push_str(", ");
        }
        out.push('\n');
    }
    out.push_str("\n---\n");
    for pronounciation in e.reading_elements() {
        out.push_str(pronounciation.text);
        out.push_str(", ");
    }
    out
}
