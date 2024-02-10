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
    std::time::Duration,
};

mod appstate;
mod conv;
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
    match IpcState::read() {
        Ok(state) => match state {
            IpcState::Visible => {
                eprintln!("Visible client already running. Exiting.");
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
    }));
    let mut rw = rw_create();
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
        if !rw.is_open() {
            match IpcState::read().unwrap() {
                IpcState::Visible => panic!("Visible state set, even though window is not visible"),
                IpcState::Hidden => {}
                IpcState::ShowRequested => {
                    rw = rw_create();
                    IpcState::Visible.write().unwrap();
                    continue;
                }
            }
            std::thread::sleep(Duration::from_millis(500));
            continue;
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
            rw.close();
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

fn rw_create() -> RenderWindow {
    RenderWindow::new(
        WIN_DIMS.to_sf_video_mode(),
        "Simple Kana Input",
        Style::DEFAULT,
        &ContextSettings::default(),
    )
}
