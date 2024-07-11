mod lumao_app;

use eframe::egui;

fn main()->eframe::Result{
    env_logger::init();
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default().with_inner_size([540.0,360.0]),
        ..Default::default()
    };
    eframe::run_native(
        "lumao statistic",
        options,
        Box::new(|cc|{
            Ok(Box::<lumao_app::LumaoApp>::default())
        })
    )
}
