use {
    crate::appstate::{AppState, UiState},
    egui_sfml::egui,
};

pub fn dict_ui(ui: &mut egui::Ui, app: &mut AppState) {
    if ui.link("Back").clicked() {
        app.ui_state = UiState::Input;
    }
}
