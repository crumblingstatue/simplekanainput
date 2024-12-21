#![feature(array_try_from_fn, let_chains)]

#[cfg(feature = "ipc")]
use existing_instance::{Endpoint, Msg};
use {appstate::AppState, std::sync::Arc};

mod appstate;
mod conv;
mod detect_edit;
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

#[cfg(not(target_arch = "wasm32"))]
const IS_WEB: bool = false;
#[cfg(target_arch = "wasm32")]
const IS_WEB: bool = true;

pub struct WinDims {
    w: u16,
    h: u16,
}

const WIN_DIMS: WinDims = WinDims { w: 640, h: 512 };

#[cfg(feature = "ipc")]
const IPC_FOCUS: Msg = Msg::Num(0);
#[cfg(feature = "ipc")]
const IPC_QUIT: Msg = Msg::Num(1);
#[cfg(feature = "ipc")]
const IPC_CHANNEL_ID: &str = "simple-kana-input";

fn main() {
    #[cfg(feature = "ipc")]
    let listener = match existing_instance::establish_endpoint(IPC_CHANNEL_ID, true).unwrap() {
        Endpoint::New(listener) => listener,
        Endpoint::Existing(mut stream) => {
            let ipc_replace = matches!(std::env::args().nth(1).as_deref(), Some("--replace"));
            if ipc_replace {
                stream.send(IPC_QUIT);
                match existing_instance::wait_to_be_new(IPC_CHANNEL_ID, true, 100, 2000) {
                    Ok(listener) => listener,
                    Err(e) => {
                        eprintln!("Error trying to replace existing instance: {e}.\n Giving up.");
                        return;
                    }
                }
            } else {
                stream.send(IPC_FOCUS);
                return;
            }
        }
    };

    let mut app = AppState::new(
        #[cfg(feature = "ipc")]
        listener,
    )
    .unwrap();

    let mut font_defs = egui::FontDefinitions::default();
    font_defs.font_data.insert(
        "ipag".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../NotoSansJP-VariableFont_wght.ttf"
        ))),
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
}
