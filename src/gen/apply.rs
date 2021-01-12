use std::path::PathBuf;
use anyhow::Result;
use crate::gen::generate;
use crate::gen::palette;
use crate::gen::write;
use crate::gen::execute;
use crate::scheme::*;
use crate::fun::text;

pub fn write_colors(scheme: &mut SCHEME, old: bool) -> Result<()> {
    if old {
        if let Some(cachepath) = scheme.cache().clone() {
            let mut palette_temp = PathBuf::from(&cachepath); palette_temp.push("palette");
            scheme.set_pigments(Some(text::lines_to_vec(palette_temp)));

            let mut wall_temp = PathBuf::from(&cachepath); wall_temp.push("wallpaper");
            if let Ok(content) = text::file_to_string(wall_temp) {
                scheme.set_image(Some(content));
            }

            let mut theme_temp = PathBuf::from(&cachepath); theme_temp.push("theme");
            if let Ok(content) = text::file_to_string(theme_temp) {
                scheme.set_theme(Some(content));
            }
        }
    } else {
        let wallpaper = scheme.walldir().clone().unwrap();
        if scheme.image().is_none() {
            scheme.set_image(Some(text::random_image(&wallpaper)));
        }

        let palette: Vec<String>;
        if let Some(content) = scheme.palette() {
            match content.as_str() {
                "pigment" => {
                    palette = palette::palette_from_image(scheme.image().clone().unwrap());
                    text::write_temp_file("lule_palette", palette.join("\n").as_bytes());
                    scheme.set_pigments(Some(palette));
                },
                _ => unreachable!(),
            };
        }
    }

    let allcolors = generate::get_all_colors(scheme);
    scheme.set_colors(Some(allcolors));

    let values = write::output_to_json(scheme, false);
    write::write_temp(&scheme);
    write::write_cache(&scheme);
    write::write_cache_json(scheme, values);
    if let Some(_) = scheme.scripts() {
        execute::command_execution(scheme);
    }
    Ok(())
}
