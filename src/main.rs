mod main_window;
mod color_reflection_window;
mod utils;

use main_window::MainWindow;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(MainWindow::default()))),
    )
}