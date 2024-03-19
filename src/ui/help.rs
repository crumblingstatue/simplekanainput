use {
    crate::{
        appstate::{AppState, UiState},
        egui,
    },
    egui_commonmark::{CommonMarkCache, CommonMarkViewer},
};

pub fn help_ui(ui: &mut egui::Ui, app: &mut AppState) {
    ui.horizontal(|ui| {
        if ui.link("Back (Esc)").clicked() || ui.input(|inp| inp.key_pressed(egui::Key::Escape)) {
            app.ui_state = UiState::Input;
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.hyperlink("https://github.com/crumblingstatue/simplekanainput");
        });
    });
    ui.separator();
    egui::ScrollArea::vertical().show(ui, |ui| {
        let mut cache = CommonMarkCache::default();
        CommonMarkViewer::new("help").show(ui, &mut cache, include_str!("../../Help.md"));
    });
}
