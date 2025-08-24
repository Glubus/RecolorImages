use eframe::egui;
use image::{ImageReader, RgbaImage};
use std::path::PathBuf;
use rfd::FileDialog;

fn recolor(image: &mut RgbaImage, target: (f32, f32, f32), blend: f32) {
    for pixel in image.pixels_mut() {
        let image::Rgba([r, g, b, a]) = *pixel;
        let r = ((1.0 - blend) * (r as f32 / 255.0) + blend * target.0).clamp(0.0, 1.0);
        let g = ((1.0 - blend) * (g as f32 / 255.0) + blend * target.1).clamp(0.0, 1.0);
        let b = ((1.0 - blend) * (b as f32 / 255.0) + blend * target.2).clamp(0.0, 1.0);
        *pixel = image::Rgba([(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, a]);
    }
}

struct ImageData {
    path: PathBuf,
    original: RgbaImage,
    recolored: RgbaImage,
    original_texture: Option<eframe::egui::TextureHandle>,
    recolored_texture: Option<eframe::egui::TextureHandle>,
    color: egui::Color32,
    blend: f32,
}

struct ImageApp {
    images: Vec<ImageData>,
    apply_to_all: bool,
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
            // Header
            ui.horizontal(|ui| {
                if ui.button("Charger Images").clicked() {
                    if let Some(paths) = FileDialog::new().pick_files() {
                        for path in paths {
                            if let Ok(img) = (|| -> Result<_, image::ImageError> {
                                let img = ImageReader::open(&path)?.decode()?;
                                Ok(img)
                            })() {
                                let rgba = img.to_rgba8();
                                let size = [rgba.width() as usize, rgba.height() as usize];
                                let original_tex = Some(ctx.load_texture(
                                    path.to_string_lossy() + "_orig",
                                    egui::ColorImage::from_rgba_unmultiplied(size, &rgba.as_raw()),
                                    Default::default(),
                                ));

                                self.images.push(ImageData {
                                    path,
                                    original: rgba.clone(),
                                    recolored: rgba.clone(),
                                    original_texture: original_tex,
                                    recolored_texture: None,
                                    color: egui::Color32::from_rgb(255, 0, 0),
                                    blend: 0.7,
                                });
                            }
                        }
                    }
                }

                ui.checkbox(&mut self.apply_to_all, "Appliquer à toutes");

                if ui.button("Sauvegarder toutes les images").clicked() {
                    if let Some(folder) = FileDialog::new().pick_folder() {
                        for img_data in &self.images {
                            let filename = img_data.path.file_name().unwrap_or_else(|| std::ffi::OsStr::new("output.png"));
                            let save_path = folder.join(filename);
                            if let Err(err) = img_data.recolored.save(&save_path) {
                                eprintln!("Erreur en sauvegardant {}: {}", save_path.display(), err);
                            }
                        }
                        println!("Toutes les images recolorisées ont été sauvegardées !");
                    }
                }
            });

            ui.separator();

            // Boucle sur toutes les images
            for i in 0..self.images.len() {
                // Prendre une copie des valeurs pour l'UI
                let color_copy = self.images[i].color;
                let blend_copy = self.images[i].blend;
            
                ui.horizontal(|ui| {
                    let (orig_tex, recolor_tex) = {
                        let img = &self.images[i];
                        (img.original_texture.clone(), img.recolored_texture.clone())
                    };
            
                    // Preview
                    ui.vertical(|ui| {
                        if let Some(tex) = &orig_tex { ui.label("Original"); ui.image(tex); }
                        if let Some(tex) = &recolor_tex { ui.label("Recoloré"); ui.image(tex); }
                    });
            
                    // Paramètres
                    ui.vertical(|ui| {
                        let mut rgb = [color_copy.r(), color_copy.g(), color_copy.b()];
                        ui.color_edit_button_srgb(&mut rgb);
                        let color_updated = egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2]);
            
                        let mut blend_updated = blend_copy;
                        ui.add(egui::Slider::new(&mut blend_updated, 0.0..=1.0).text("Blend"));
            
                        // On stocke les nouvelles valeurs après la closure
                        self.images[i].color = color_updated;
                        self.images[i].blend = blend_updated;
                    });
                });
            
                // Après la UI, appliquer recolor
                if self.apply_to_all {
                    let color = self.images[i].color;
                    let blend = self.images[i].blend;
                    for img_all in &mut self.images {
                        img_all.recolored = img_all.original.clone();
                        recolor(
                            &mut img_all.recolored,
                            (color.r() as f32 / 255.0, color.g() as f32 / 255.0, color.b() as f32 / 255.0),
                            img_all.blend,
                        );
                        let size = [img_all.recolored.width() as usize, img_all.recolored.height() as usize];
                        img_all.recolored_texture = Some(ctx.load_texture(
                            img_all.path.to_string_lossy() + "_recolor",
                            egui::ColorImage::from_rgba_unmultiplied(size, img_all.recolored.as_raw()),
                            Default::default(),
                        ));
                    }
                } else {
                    let color = self.images[i].color;
                    let blend = self.images[i].blend;
                    let img = &mut self.images[i];
                    img.recolored = img.original.clone();
                    recolor(
                        &mut img.recolored,
                        (color.r() as f32 / 255.0, color.g() as f32 / 255.0, color.b() as f32 / 255.0),
                        blend,
                    );
                    let size = [img.recolored.width() as usize, img.recolored.height() as usize];
                    img.recolored_texture = Some(ctx.load_texture(
                        img.path.to_string_lossy() + "_recolor",
                        egui::ColorImage::from_rgba_unmultiplied(size, img.recolored.as_raw()),
                        Default::default(),
                    ));
                }
            }
            ui.separator();
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Recolor Images Auto",
        options,
        Box::new(|_cc| Ok(Box::new(ImageApp::default()))),
    )
    .unwrap();
}
