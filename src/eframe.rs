use {
    crate::appstate::AppState,
    eframe::egui::{FontDefinitions, ViewportCommand},
};

impl eframe::App for AppState {
    fn ui(&mut self, ctx: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        if !crate::ui::update(ctx, self) {
            // On the eframe backend, the app automatically quits if the window is closed
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
        ctx.request_repaint();
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn do_eframe_event_loop(font_defs: FontDefinitions, style: eframe::egui::Style, app: AppState) {
    let opts = eframe::NativeOptions {
        window_builder: Some(Box::new(|wb| {
            use crate::WIN_DIMS;

            wb.with_inner_size((WIN_DIMS.w as f32, WIN_DIMS.h as f32))
        })),
        ..Default::default()
    };
    eframe::run_native(
        "Simple Kana Input (eframe)",
        opts,
        Box::new(|cc| {
            cc.egui_ctx.set_global_style(style);
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
