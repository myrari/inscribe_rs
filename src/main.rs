struct App {}

impl App {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl Default for App {
    fn default() -> Self {
        App {}
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // theme widget
            egui::widgets::global_theme_preference_buttons(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello from inscribe_rs");
        });
    }
}

fn main() {
    env_logger::init();

    let _ = eframe::run_native(
        "inscribers",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder {
                title: Some("wip-inscribers".into()),
                ..Default::default()
            },
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    );
}
