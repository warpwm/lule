use crate::fun::text;
use crate::scheme::*;
use anyhow::Result;
use std::path::PathBuf;
use tera::{Context, Tera};

fn generate_template(original: PathBuf, replaced: PathBuf, scheme: &Scheme) -> Result<()> {
    let mut content = String::new();
    if let Ok(cont) = text::file_to_string(original) {
        content = cont;
    }

    let mut tera = Tera::default();
    tera.add_raw_template("template", &content)?;

    let mut context = Context::new();

    if let Some(colors) = scheme.colors() {
        for (i, color) in colors.iter().enumerate() {
            let name = format!("color{}", i);
            context.insert(name, &color.to_rgb_hex_string(false));
        }
        context.insert("background", &colors[0].to_rgb_hex_string(false));
        context.insert("foreground", &colors[15].to_rgb_hex_string(false));
        context.insert("cursor", &colors[1].to_rgb_hex_string(false));
        context.insert("accent", &colors[1].to_rgb_hex_string(false));
    }

    if let Some(wallpaper) = scheme.image() {
        context.insert("wallpaper", &wallpaper);
    }
    if let Some(theme) = scheme.theme() {
        context.insert("theme", &theme);
    }

    let new_content = tera.render("template", &context)?;
    text::write_to_file(replaced, new_content.as_bytes());

    Ok(())
}

pub fn pattern_generation(scheme: &mut Scheme) -> Result<()> {
    if let Some(patterns) = scheme.patterns() {
        for p in patterns.iter() {
            if std::fs::metadata(&p.0).is_ok() && std::fs::metadata(&p.1).is_ok() {
                generate_template(PathBuf::from(&p.0), PathBuf::from(&p.1), scheme)?;
                println!("generating :{} into: {}", p.0, p.1)
            } else {
                // TODO: better error handle
                println!("{} or {} is not a valid file", p.0, p.1)
            }
        }
    }

    Ok(())
}
