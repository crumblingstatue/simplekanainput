use {
    crate::appstate::{AppState, UiState},
    egui_colors::{Colorix, tokens::ThemeColor},
    egui_sfml::egui,
    rand::Rng,
};

pub fn theme_ui(ui: &mut egui::Ui, app: &mut AppState) {
    if ui.link("Back").clicked() || ui.input(|inp| inp.key_pressed(egui::Key::Escape)) {
        app.ui_state = UiState::Input;
    }
    let colorix = app
        .colorix
        .get_or_insert_with(|| Colorix::global(ui.ctx(), [ThemeColor::Custom([0, 0, 0]); 12]));
    ui.group(|ui| {
        colorix.ui_combo_12(ui, true);
    });
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
    });
}
