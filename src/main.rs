use inscribe_rs::app::App;

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
