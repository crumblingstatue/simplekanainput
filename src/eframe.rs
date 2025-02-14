use {
    crate::appstate::AppState,
    eframe::egui::{FontDefinitions, ViewportCommand},
};

impl eframe::App for AppState {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if !crate::ui::update(ctx, self) {
            // On the eframe backend, the app automatically quits if the window is closed
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
        ctx.request_repaint();
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn do_eframe_event_loop(font_defs: FontDefinitions, style: eframe::egui::Style, app: AppState) {
    eframe::run_native(
        "Simple Kana Input (eframe)",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_style(style);
            cc.egui_ctx.set_fonts(font_defs);
            Ok(Box::new(app))
        }),
    )
    .unwrap();
}

#[cfg(target_arch = "wasm32")]
pub fn do_eframe_event_loop(font_defs: FontDefinitions, style: eframe::egui::Style, app: AppState) {
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| {
                    cc.egui_ctx.set_style(style);
                    cc.egui_ctx.set_fonts(font_defs);
                    Box::new(app)
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}
