use eframe::egui;
use crate::ui::{image_panel::{draw_image_panel_single, apply_to_all_images}, toolbar::toolbar};

pub struct ImageData {
    pub path: std::path::PathBuf,
    pub original: image::RgbaImage,
    pub recolored: image::RgbaImage,
    pub original_texture: Option<eframe::egui::TextureHandle>,
    pub recolored_texture: Option<eframe::egui::TextureHandle>,
    pub color: egui::Color32,
    pub blend: f32,
}

pub struct ImageApp {
    pub images: Vec<ImageData>,
    pub apply_to_all: bool,
}

impl Default for ImageApp {
    fn default() -> Self {
        Self {
            images: vec![],
            apply_to_all: false,
        }
    }
}

impl eframe::App for ImageApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            toolbar(ui, self, ctx);

            ui.separator();

            for i in 0..self.images.len() {
                draw_image_panel_single(ui, &mut self.images[i], ctx);
            }

            if self.apply_to_all && !self.images.is_empty() {
                let color = self.images[0].color;
                apply_to_all_images(&mut self.images, color, ctx);
            }

            ui.separator();
        });
    }
}
