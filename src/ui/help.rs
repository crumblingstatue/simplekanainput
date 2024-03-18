use {
    crate::{
        appstate::{AppState, UiState},
        egui,
    },
    egui_commonmark::{CommonMarkCache, CommonMarkViewer},
};

pub fn help_ui(ui: &mut egui::Ui, app: &mut AppState) {
    if ui.link("Back").clicked() {
        app.ui_state = UiState::Input;
    }
    ui.separator();
    egui::ScrollArea::vertical().show(ui, |ui| {
        let mut cache = CommonMarkCache::default();
        CommonMarkViewer::new("help").show(ui, &mut cache, include_str!("../../Help.md"));
    });
}
