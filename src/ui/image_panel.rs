use eframe::egui;
use crate::ui::image_app::ImageData;
use crate::services::image_service::recolor;

/// Dessine l'interface d'une seule image et applique le recolor
pub fn draw_image_panel_single(ui: &mut egui::Ui, image: &mut ImageData, ctx: &egui::Context) {
    let color_copy = image.color;
    let blend_copy = image.blend;

    ui.horizontal(|ui| {
        let (orig_tex, recolor_tex) = (image.original_texture.clone(), image.recolored_texture.clone());

        // Preview
        ui.vertical(|ui| {
            if let Some(tex) = &orig_tex { ui.label("Original"); ui.image(tex); }
            if let Some(tex) = &recolor_tex { ui.label("Recoloré"); ui.image(tex); }
        });

        // Paramètres couleur et blend
        ui.vertical(|ui| {
            let mut rgb = [color_copy.r(), color_copy.g(), color_copy.b()];
            ui.color_edit_button_srgb(&mut rgb);
            let color_updated = egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2]);

            let mut blend_updated = blend_copy;
            ui.add(egui::Slider::new(&mut blend_updated, 0.0..=1.0).text("Blend"));

            image.color = color_updated;
            image.blend = blend_updated;
        });
    });

    // Appliquer recolor à l'image courante
    image.recolored = image.original.clone();
    recolor(
        &mut image.recolored,
        (image.color.r() as f32 / 255.0, image.color.g() as f32 / 255.0, image.color.b() as f32 / 255.0),
        image.blend,
    );

    let size = [image.recolored.width() as usize, image.recolored.height() as usize];
    image.recolored_texture = Some(ctx.load_texture(
        image.path.to_string_lossy() + "_recolor",
        egui::ColorImage::from_rgba_unmultiplied(size, image.recolored.as_raw()),
        Default::default(),
    ));
}

/// Applique la couleur d'une image à toutes les autres
pub fn apply_to_all_images(images: &mut [ImageData], color: egui::Color32, ctx: &egui::Context) {
    for img in images.iter_mut() {
        img.recolored = img.original.clone();
        recolor(
            &mut img.recolored,
            (color.r() as f32 / 255.0, color.g() as f32 / 255.0, color.b() as f32 / 255.0),
            img.blend,
        );
        let size = [img.recolored.width() as usize, img.recolored.height() as usize];
        img.recolored_texture = Some(ctx.load_texture(
            img.path.to_string_lossy() + "_recolor",
            egui::ColorImage::from_rgba_unmultiplied(size, img.recolored.as_raw()),
            Default::default(),
        ));
    }
}
