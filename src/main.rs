#![feature(array_try_from_fn)]

use {
    crate::ipc::IpcState,
    appstate::AppState,
    egui_sfml::{
        egui::{self, FontData, FontFamily},
        sfml::{
            graphics::{Rect, RenderTarget, RenderWindow, View},
            system::Vector2,
            window::{ContextSettings, Event, Style, VideoMode},
        },
        SfEgui,
    },
    sfml_xt::graphics::RenderWindowExt,
    std::{
        backtrace::Backtrace,
        time::{Duration, Instant},
    },
};

mod appstate;
mod conv;
mod detect_edit;
mod ipc;
mod kana;
mod kanji;
mod radicals;
mod segment;
mod ui;

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

const WIN_DIMS: WinDims = WinDims { w: 640, h: 512 };

fn main() {
    let ipc_replace = matches!(std::env::args().nth(1).as_deref(), Some("--replace"));
    if ipc_replace {
        eprintln!("Replacing running instance...");
        eprintln!("{:?}", IpcState::QuitRequested.write());
        // Wait for quit of original
        let wait_start = Instant::now();
        loop {
            std::thread::sleep(Duration::from_millis(100));
            if dbg!(IpcState::read()).is_err() {
                break;
            }
            // Time out
            if wait_start.elapsed().as_millis() > 800 {
                eprintln!("Timed out, starting normally...");
                IpcState::remove().unwrap();
                break;
            }
        }
    }
    match IpcState::read() {
        Ok(state) => match state {
            IpcState::Visible => {
                eprintln!("{:?}", IpcState::ShowRequested.write());
                eprintln!("Visible client already running. (Sent show request for focus)");
                return;
            }
            IpcState::Hidden => {
                eprintln!("Hidden client, setting show request state.");
                eprintln!("{:?}", IpcState::ShowRequested.write());
                return;
            }
            IpcState::ShowRequested => {
                eprintln!("Show requested already in progress. Exiting.");
                return;
            }
            IpcState::QuitRequested => {
                eprintln!("お前はもう死んでいる。何?");
                return;
            }
        },
        Err(e) => {
            eprintln!("Error reading IPC state: {e}\nStarting normally.");
        }
    }
    // We start out visible
    IpcState::Visible.write().unwrap();
    std::panic::set_hook(Box::new(|info| {
        rfd::MessageDialog::new()
            .set_title("Panic")
            .set_description(&info.to_string())
            .show();
        let bt = Backtrace::capture();
        eprintln!("{bt}");
        eprintln!("remove ipc result: {:?}", IpcState::remove());
    }));
    let mut rw = RenderWindow::new(
        WIN_DIMS.to_sf_video_mode(),
        "Simple Kana Input",
        Style::DEFAULT,
        &ContextSettings::default(),
    );
    rw.center();
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
        FontData::from_static(include_bytes!("../NotoSansJP-VariableFont_wght.ttf")),
    );
    font_defs
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .push("ipag".to_owned());
    sf_egui.context().set_fonts(font_defs);
    let mut style = egui::Style::default();
    for (text_style, font_id) in style.text_styles.iter_mut() {
        let size = match *text_style {
            egui::TextStyle::Small => 16.0,
            egui::TextStyle::Body => 18.0,
            egui::TextStyle::Monospace => 16.0,
            egui::TextStyle::Button => 16.0,
            egui::TextStyle::Heading => 21.0,
            egui::TextStyle::Name(_) => todo!(),
        };
        font_id.size = size;
    }
    sf_egui.context().set_style(style);

    loop {
        match IpcState::read().unwrap() {
            IpcState::Visible => {}
            IpcState::Hidden => {}
            IpcState::ShowRequested => {
                // Need this set_visible(false) trick to refocus a visible, but unfocused window.
                // Requesting focus just flashes the tray icon.
                rw.set_visible(false);
                rw.set_visible(true);
                rw.center();
                IpcState::Visible.write().unwrap();
            }
            IpcState::QuitRequested => break,
        }
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
            .do_frame(|ctx| {
                egui::CentralPanel::default().show(ctx, |ui| match app.ui_state {
                    appstate::UiState::Input => ui::input_ui(ui, &mut app),
                    appstate::UiState::Dict => ui::dict_ui(ui, &mut app),
                    appstate::UiState::Kanji => ui::kanji_ui(ui, &mut app),
                });
            })
            .unwrap();
        if app.hide_requested {
            IpcState::Hidden.write().unwrap();
            rw.set_visible(false);
            app.hide_requested = false;
        }
        if app.quit_requested {
            break;
        }
        sf_egui.draw(&mut rw, None);
        rw.display();
    }
    eprintln!("{:?}", IpcState::remove());
}
