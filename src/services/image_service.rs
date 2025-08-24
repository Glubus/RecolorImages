use image::{ImageReader, RgbaImage};
use std::path::PathBuf;

/// Recolore une image selon une couleur cible et un facteur de blend
pub fn recolor(image: &mut RgbaImage, target: (f32, f32, f32), blend: f32) {
    for pixel in image.pixels_mut() {
        let image::Rgba([r, g, b, a]) = *pixel;
        let r = ((1.0 - blend) * (r as f32 / 255.0) + blend * target.0).clamp(0.0, 1.0);
        let g = ((1.0 - blend) * (g as f32 / 255.0) + blend * target.1).clamp(0.0, 1.0);
        let b = ((1.0 - blend) * (b as f32 / 255.0) + blend * target.2).clamp(0.0, 1.0);
        *pixel = image::Rgba([(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, a]);
    }
}

/// Charge une image depuis un chemin et retourne un RgbaImage
pub fn load_image(path: &PathBuf) -> Result<RgbaImage, image::ImageError> {
    let img = ImageReader::open(path)?.decode()?;
    Ok(img.to_rgba8())
}
