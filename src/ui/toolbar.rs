use crate::services::image_service::load_image;
use crate::ui::image_app::ImageApp;
use eframe::egui;
use rfd::FileDialog;

pub fn toolbar(ui: &mut egui::Ui, app: &mut ImageApp, ctx: &egui::Context) {
    ui.horizontal(|ui| {
        if ui.button("Charger Images").clicked() {
            if let Some(paths) = FileDialog::new().pick_files() {
                for path in paths {
                    if let Ok(rgba) = load_image(&path) {
                        let size = [rgba.width() as usize, rgba.height() as usize];
                        let original_tex = Some(ctx.load_texture(
                            path.to_string_lossy() + "_orig",
                            egui::ColorImage::from_rgba_unmultiplied(size, &rgba.as_raw()),
                            Default::default(),
                        ));

                        app.images.push(crate::ui::image_app::ImageData {
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

        ui.checkbox(&mut app.apply_to_all, "Appliquer à toutes");

        if ui.button("Sauvegarder toutes les images").clicked() {
            if let Some(folder) = FileDialog::new().pick_folder() {
                for img_data in &app.images {
                    let filename = img_data
                        .path
                        .file_name()
                        .unwrap_or_else(|| std::ffi::OsStr::new("output.png"));
                    let save_path = folder.join(filename);
                    if let Err(err) = img_data.recolored.save(&save_path) {
                        eprintln!("Erreur en sauvegardant {}: {}", save_path.display(), err);
                    }
                }
                println!("Toutes les images recolorisées ont été sauvegardées !");
            }
        }
    });
}
