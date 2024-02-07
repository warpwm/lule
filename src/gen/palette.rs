use colored::*;
use std::path::PathBuf;

use crate::fun::text;
use crate::gen::kmeans;

use super::hex::color_from_hex;

pub fn palette_from_image(image: String) -> Vec<String> {
    let colors_lab = kmeans::pigments(&image, 16, Some(300)).unwrap_or_else(|err| {
        eprintln!(
            "{} Problem creating palette -> {}",
            "error:".red().bold(),
            err
        );
        std::process::exit(1);
    });

    let mut colors = Vec::new();
    for (color, _) in colors_lab.iter() {
        let lab_color = pastel::Color::from_lab(color.l, color.a, color.b, 1.into());
        colors.push(lab_color.clone().to_rgb_hex_string(true));
    }
    colors
}

pub fn colors_from_file(
    filename: PathBuf,
) -> Result<Vec<pastel::Color>, Box<dyn std::error::Error>> {
    let mut colors = Vec::new();
    for line in text::lines_to_vec(filename) {
        colors.push(color_from_hex(&line));
    }
    Ok(colors)
}
