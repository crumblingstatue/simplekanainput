#![feature(iterator_try_collect)]

mod appstate;
mod conv;
mod ui;

use {
    appstate::AppState,
    egui_sfml::{
        egui::{self, FontData, FontFamily},
        sfml::{
            graphics::RenderWindow,
            system::Vector2,
            window::{ContextSettings, Event, Style, VideoMode},
        },
        SfEgui,
    },
    std::time::Duration,
};

pub struct WinDims {
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
    let mut app = AppState::new().unwrap();
    rw.set_framerate_limit(60);
    rw.set_position(Vector2::new(
        1920 / 2 - app.half_dims.w as i32,
        1080 / 2 - app.half_dims.h as i32,
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

    while rw.is_open() {
        while let Some(ev) = rw.poll_event() {
            sf_egui.add_event(&ev);

            if let Event::Closed = ev {
                rw.close();
            }
        }
        sf_egui
            .do_frame(|ctx| {
                egui::CentralPanel::default().show(ctx, |ui| ui::central_panel_ui(ui, &mut app));
            })
            .unwrap();
        if app.quit_requested {
            rw.close();
        }
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
