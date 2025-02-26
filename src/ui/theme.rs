use {
    crate::{
        appstate::{AppState, UiState},
        egui,
    },
    egui_colors::{Colorix, tokens::ThemeColor},
    rand::Rng,
};

pub fn theme_ui(ui: &mut egui::Ui, app: &mut AppState) {
    if ui.link("Back").clicked() || ui.input(|inp| inp.key_pressed(egui::Key::Escape)) {
        app.ui_state = UiState::Input;
    }
    let colorix = match &mut app.colorix {
        Some(colorix) => colorix,
        None => {
            if ui.button("Enable custom theme").clicked() {
                app.colorix.insert(Colorix::global(
                    ui.ctx(),
                    [ThemeColor::Custom([0, 0, 0]); 12],
                ))
            } else {
                return;
            }
        }
    };
    ui.group(|ui| {
        colorix.ui_combo_12(ui, true);
    });
    let mut disable = false;
    ui.horizontal(|ui| {
        colorix.themes_dropdown(ui, None, false);
        ui.label("Light/dark");
        colorix.light_dark_toggle_button(ui, 20.0);
        if ui.button("Randomize").clicked() {
            let mut rng = rand::rng();
            *colorix = Colorix::global(
                ui.ctx(),
                std::array::from_fn(|_| ThemeColor::Custom(std::array::from_fn(|_| rng.random()))),
            );
        }
        if ui.button("Disable custom theme").clicked() {
            disable = true;
        }
    });
    if disable {
        ui.ctx().set_visuals(egui::Visuals::dark());
        app.colorix = None;
    }
}
