use colored::*;
use std::path::PathBuf;

use crate::gen::kmeans;
use crate::fun::text;

pub fn palette_from_image(image: String) -> Vec<String> {
    let colors_lab = kmeans::pigments(&image, 16, Some(300))
        .unwrap_or_else(|err| {
            eprintln!("{} {} {}",
                "error:".red().bold(), 
                "Problem creating palette ->", 
                err);
            std::process::exit(1);
        });

    let mut colors = Vec::new();
    for (color, _) in colors_lab.iter() {
        let lab_color = pastel::Color::from_lab(
                color.l.into(),
                color.a.into(),
                color.b.into(),
                1.into());
        colors.push(pastel::Color::from(lab_color.clone()).to_rgb_hex_string(true));
    }
    colors
}

pub fn colors_from_file(filename: PathBuf) -> Result<Vec<pastel::Color>, Box<dyn std::error::Error>> {
    let mut colors = Vec::new();
    for line in text::lines_to_vec(filename) {
        colors.push(pastel::Color::from_hex(&line));
    }
    Ok(colors)
}
