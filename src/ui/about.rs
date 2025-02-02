use crate::{
    appstate::{AppState, UiState},
    egui,
};

macro_rules! optenv {
    ($name:literal) => {
        option_env!($name).unwrap_or("<unavailable>").to_string()
    };
}

pub fn about_ui(ui: &mut egui::Ui, app: &mut AppState) {
    ui.horizontal(|ui| {
        if ui.link("Back (Esc)").clicked() || ui.input(|inp| inp.key_pressed(egui::Key::Escape)) {
            app.ui_state = UiState::Input;
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.hyperlink("https://github.com/crumblingstatue/simplekanainput");
        });
    });
    ui.separator();
    ui.label(format!(
        "Simple kana input version {}",
        optenv!("CARGO_PKG_VERSION")
    ));
    let mut job = egui::text::LayoutJob::default();
    macro_rules! pair {
        ($label:literal, $envname:literal) => {
            job.append($label, 0.0, egui::text::TextFormat::default());
            job.append(
                &optenv!($envname),
                0.0,
                egui::text::TextFormat {
                    color: egui::Color32::WHITE,
                    ..Default::default()
                },
            );
        };
    }
    pair!("Features: ", "VERGEN_CARGO_FEATURES");
    pair!("\nGit SHA: ", "VERGEN_GIT_SHA");
    pair!("\nCommit time: ", "VERGEN_GIT_COMMIT_TIMESTAMP");
    pair!("\nBuild time: ", "VERGEN_BUILD_TIMESTAMP");
    pair!("\nTarget: ", "VERGEN_CARGO_TARGET_TRIPLE");
    pair!("\nDebug: ", "VERGEN_CARGO_DEBUG");
    pair!("\nOpt-level: ", "VERGEN_CARGO_OPT_LEVEL");
    pair!("\nBuilt with rustc: ", "VERGEN_RUSTC_SEMVER");
    ui.label(job);
}
