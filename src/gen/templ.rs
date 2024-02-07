use crate::fun::text;
use crate::scheme::*;
use anyhow::Result;
use std::collections::BTreeMap;
use std::path::PathBuf;
use handlebars::Handlebars;

fn generate_template(original: PathBuf, replaced: PathBuf, scheme: &Scheme) -> Result<()> {

    let mut content = String::new();
    if let Ok(cont) = text::file_to_string(original) {
        content = cont;
    }

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("template", &content)?;

    let mut data = BTreeMap::new();

    if let Some(colors) = scheme.colors() {
        for (i, color) in colors.iter().enumerate() {
            let name = format!("color{}", i);
            data.insert(name, color.to_rgb_hex_string(false));
        }
        data.insert("background".to_string(), colors[0].to_rgb_hex_string(false));
        data.insert("foreground".to_string(), colors[15].to_rgb_hex_string(false));
        data.insert("cursor".to_string(), colors[1].to_rgb_hex_string(false));
        data.insert("accent".to_string(), colors[1].to_rgb_hex_string(false));
    }

    if let Some(wallpaper) = scheme.image() {
        data.insert("wallpaper".to_string(), wallpaper.clone());
    }
    if let Some(theme) = scheme.theme() {
        data.insert("theme".to_string(), theme.clone());
    }

    let new_content = handlebars.render("template", &data)?;
    
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
                //TODO: better error handle
                println!("{} or {} is not a valid file", p.0, p.1)
            }
        }
    }

    Ok(())
}
