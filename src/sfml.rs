use {
    crate::{appstate::AppState, WinDims, WIN_DIMS},
    egui_sfml::{
        egui::{self, FontDefinitions},
        sfml::{
            graphics::{Rect, RenderTarget as _, RenderWindow, View},
            window::{Event, Style, VideoMode},
        },
        SfEgui,
    },
    sfml_xt::graphics::RenderWindowExt as _,
};

impl WinDims {
    fn to_sf_video_mode(&self) -> VideoMode {
        VideoMode {
            width: self.w.into(),
            height: self.h.into(),
            bits_per_pixel: 32,
        }
    }
}

pub fn do_sfml_event_loop(font_defs: FontDefinitions, style: egui::Style, app: &mut AppState) {
    let mut rw = RenderWindow::new(
        WIN_DIMS.to_sf_video_mode(),
        "Simple Kana Input",
        Style::DEFAULT,
        &egui_sfml::sfml::window::ContextSettings::default(),
    );
    rw.center();
    rw.set_vertical_sync_enabled(true);
    let mut sf_egui = SfEgui::new(&rw);
    sf_egui.context().set_fonts(font_defs);
    sf_egui.context().set_style(style);
    let mut quit = false;

    while !quit {
        while let Some(ev) = rw.poll_event() {
            sf_egui.add_event(&ev);

            match ev {
                Event::Closed => app.hide_requested = true,
                Event::Resized { width, height } => rw.set_view(&View::from_rect(Rect::new(
                    0.,
                    0.,
                    width as f32,
                    height as f32,
                ))),
                _ => {}
            }
        }
        sf_egui
            .do_pass(&mut rw, |ctx| {
                if !crate::ui::update(ctx, app) {
                    quit = true;
                }
            })
            .unwrap();

        sf_egui.draw(&mut rw, None);
        rw.display();
    }
}
