use {
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
mod kana;
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

const IPC_PATH: &str = "/dev/shm/simplekanainput.ipc.dat";

fn main() {
    match std::fs::read_to_string(IPC_PATH) {
        Ok(msg) => match msg.as_str() {
            "state_hidden" => {
                std::fs::write(IPC_PATH, "cmd_show").unwrap();
                return;
            }
            "state_visible" => {
                panic!("Window (should be) already visible");
            }
            "cmd_show" => {
                panic!("Show already in progress");
            }
            _ => {}
        },
        Err(e) => {
            eprintln!("IPC read error: {e}.\nStarting normally.")
        }
    }
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
        FontData::from_static(include_bytes!("../ipag.ttf")),
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
            match std::fs::read_to_string(IPC_PATH) {
                Ok(msg) => match msg.as_str() {
                    "cmd_show" => {
                        std::fs::write(IPC_PATH, "state_visible").unwrap();
                        rw = rw_create();
                        continue;
                    }
                    "state_hidden" => {}
                    etc => eprintln!("IPC value read: {etc}"),
                },
                Err(e) => {
                    panic!("IPC read error: {e}.");
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
                });
            })
            .unwrap();
        if app.hide_requested {
            std::fs::write(IPC_PATH, "state_hidden").unwrap();
            rw.close();
            app.hide_requested = false;
        }
        if app.quit_requested {
            break;
        }
        sf_egui.draw(&mut rw, None);
        rw.display();
    }
    eprintln!("{:?}", std::fs::remove_file(IPC_PATH));
}

fn rw_create() -> RenderWindow {
    RenderWindow::new(
        WIN_DIMS.to_sf_video_mode(),
        "Simple Kana Input",
        Style::DEFAULT,
        &ContextSettings::default(),
    )
}
