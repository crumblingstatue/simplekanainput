#![feature(array_try_from_fn)]

use {
    crate::ipc::IpcState,
    appstate::AppState,
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

#[cfg(feature = "backend-eframe")]
mod eframe;
#[cfg(feature = "backend-sfml")]
mod sfml;

#[cfg(feature = "backend-eframe")]
use ::eframe::egui;
#[cfg(feature = "backend-sfml")]
use egui_sfml::egui;

pub struct WinDims {
    w: u16,
    h: u16,
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

    let mut app = AppState::new().unwrap();

    let mut font_defs = egui::FontDefinitions::default();
    font_defs.font_data.insert(
        "ipag".to_owned(),
        egui::FontData::from_static(include_bytes!("../NotoSansJP-VariableFont_wght.ttf")),
    );
    font_defs
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .push("ipag".to_owned());
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
    #[cfg(feature = "backend-sfml")]
    crate::sfml::do_sfml_event_loop(font_defs, style, &mut app);
    #[cfg(feature = "backend-eframe")]
    crate::eframe::do_eframe_event_loop(font_defs, style, app);
    eprintln!("{:?}", IpcState::remove());
}
