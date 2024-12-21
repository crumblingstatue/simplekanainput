use {
    crate::appstate::{AppState, UiState},
    egui_colors::{tokens::ThemeColor, Colorix},
    egui_sfml::egui,
    rand::Rng,
};

pub fn theme_ui(ui: &mut egui::Ui, app: &mut AppState) {
    if ui.link("Back").clicked() {
        app.ui_state = UiState::Input;
    }
    let colorix = app
        .colorix
        .get_or_insert_with(|| Colorix::init(ui.ctx(), [ThemeColor::Custom([0, 0, 0]); 12]));
    ui.group(|ui| {
        colorix.ui_combo_12(ui, true);
    });
    colorix.themes_dropdown(ui, None, false);
    colorix.light_dark_toggle_button(ui);
    if ui.button("Randomize").clicked() {
        let mut rng = rand::thread_rng();
        *colorix = Colorix::init(
            ui.ctx(),
            std::array::from_fn(|_| ThemeColor::Custom(std::array::from_fn(|_| rng.gen()))),
        );
    }
}
