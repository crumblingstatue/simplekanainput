use {
    crate::appstate::AppState,
    eframe::{egui::FontDefinitions, NativeOptions},
};

impl eframe::App for AppState {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        if !crate::ui::update(ctx, self) {
            eprintln!("Wants to quit, I guess");
        }
    }
}

pub fn do_eframe_event_loop(font_defs: FontDefinitions, style: eframe::egui::Style, app: AppState) {
    eframe::run_native(
        "Simple Kana Input (eframe)",
        NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_style(style);
            cc.egui_ctx.set_fonts(font_defs);
            Box::new(app)
        }),
    )
    .unwrap();
}
