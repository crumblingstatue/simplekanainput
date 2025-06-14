use {
    crate::{WIN_DIMS, WinDims, appstate::AppState},
    egui_sf2g::{
        SfEgui,
        egui::{self, FontDefinitions},
        sf2g::{
            graphics::{Rect, RenderTarget as _, RenderWindow, View},
            window::{Event, Style, VideoMode},
        },
    },
    sf2g_xt::graphics::RenderWindowExt as _,
    std::time::Duration,
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
        &egui_sf2g::sf2g::window::ContextSettings::default(),
    )
    .unwrap();
    rw.center();
    rw.set_vertical_sync_enabled(true);
    let mut sf_egui = SfEgui::new(&rw);
    sf_egui.context().set_fonts(font_defs);
    sf_egui.context().set_style(style);
    let mut quit = false;

    while !quit {
        if app.hidden {
            crate::ui::handle_ipc_messages(app, sf_egui.context());
            std::thread::sleep(Duration::from_millis(250));
            continue;
        }
        while let Some(ev) = rw.poll_event() {
            sf_egui.add_event(&ev);

            match ev {
                Event::Closed => app.hide_requested = true,
                Event::Resized { width, height } => rw.set_view(
                    &View::from_rect(Rect::new(0., 0., width as f32, height as f32)).unwrap(),
                ),
                _ => {}
            }
        }
        let di = sf_egui
            .run(&mut rw, |_rw, ctx| {
                if !crate::ui::update(ctx, app) {
                    quit = true;
                }
            })
            .unwrap();

        sf_egui.draw(di, &mut rw, None);
        rw.display();
    }
}
