use anyhow::{Result, Context};
use serde_json::Value;
use std::collections::HashMap as Map;
use std::path::PathBuf;
use std::env;
use crate::scheme::*;
use crate::fun::text;

pub fn write_temp(scheme: &SCHEME) {
    let mut record = Vec::new();
    if let Some(colors) = scheme.colors() {
        for color in colors.iter() {
            record.push(format!("{}", color.to_rgb_hex_string(true)));
        }
        text::write_temp_file("lule_colors", record.join("\n").as_bytes());
    }
    if let Some(wallpaper) = scheme.image() {
        text::write_temp_file("lule_wallpaper", wallpaper.as_bytes());
    }
    if let Some(theme) = scheme.theme() {
        text::write_temp_file("lule_theme", theme.as_bytes());
    }
    let scheme_json = serde_json::to_value(&scheme).unwrap();
    let format_scheme = format!("{}", scheme_json);
    text::write_temp_file("lule_scheme", format_scheme.as_bytes());
}

pub fn write_cache(scheme: &SCHEME) {
    let cache_path = match scheme.cache() {
        Some(value) => value,
        None => ""
    };

    let lule_colors = text::pather(vec!["lule_colors"], env::temp_dir());
    let colors = text::pather(vec!["colors"], PathBuf::from(cache_path));
    text::copy_to(lule_colors, colors);

    let lule_wallpaper = text::pather(vec!["lule_wallpaper"], env::temp_dir());
    let wallpaper = text::pather(vec!["wallpaper"], PathBuf::from(cache_path));
    text::copy_to(lule_wallpaper, wallpaper);

    let lule_theme = text::pather(vec!["lule_theme"], env::temp_dir());
    let theme = text::pather(vec!["theme"], PathBuf::from(cache_path));
    text::copy_to(lule_theme, theme);

    let lule_palette = text::pather(vec!["lule_palette"], env::temp_dir());
    let theme = text::pather(vec!["palette"], PathBuf::from(cache_path));
    text::copy_to(lule_palette, theme);
}

pub fn write_cache_json(scheme: &mut SCHEME, values: Value) {
    let cache_path = match scheme.cache() {
        Some(value) => value,
        None => ""
    };
    let cache_json = text::pather(vec!["colors.json"], PathBuf::from(cache_path));
    let json_out = serde_json::to_string_pretty(&values).unwrap();
    text::write_to_file(cache_json, json_out.as_bytes());
}

pub fn output_to_json(scheme: &mut SCHEME, map: bool) -> Value {
    let mut color_map = Map::new();
    let mut color_vec = Vec::new();
    if let Some(colors) = scheme.colors() {
        for (key, color) in colors.iter().enumerate() {
            let name = "color".to_string() + &key.to_string();
            color_map.insert(name, pastel::HEX::from(color).to_string());
            color_vec.push(color.to_rgb_hex_string(true));
        }
    }
    let map_profile = ProfileMap {
        wallpaper: scheme.image().clone().unwrap(),
        theme: scheme.theme().clone().unwrap(),
        special: Special {
            background: color_vec[0].clone(),
            foreground: color_vec[15].clone(),
            cursor: color_vec[1].clone()
        },
        colors: color_map
    };
    let vec_profile = ProfileVec {
        wallpaper: scheme.image().clone().unwrap(),
        theme: scheme.theme().clone().unwrap(),
        special: Special {
            background: color_vec[0].clone(),
            foreground: color_vec[15].clone(),
            cursor: color_vec[1].clone()
        },
        colors: color_vec
    };
    if map {
        serde_json::to_value(&map_profile).unwrap()
    } else {
        serde_json::to_value(&vec_profile).unwrap()
    }
}

pub fn json_to_scheme(data: String) -> Result<SCHEME> {
    let scheme: SCHEME = serde_json::from_str(&data).context("something got fucked-up reaading json")?;
    Ok(scheme)
}
