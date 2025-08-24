#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod services;
mod ui;

use eframe::NativeOptions;
use ui::image_app::ImageApp;



fn main() {
    let options = NativeOptions::default();

    eframe::run_native(
        "Recolor Images",
        options,
        Box::new(|_cc| Ok(Box::new(ImageApp::default()))),
    )
    .unwrap();
}
