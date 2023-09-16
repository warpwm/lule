use std::path::PathBuf;
use crate::scheme::*;
use colored::*;
use dirs;

pub fn concatinate(scheme: &mut SCHEME) {
    let _home_path: PathBuf = dirs::home_dir().unwrap_or_else(|| panic!("{} {}", "error:".red().bold(), "Path of home is impossible to get"));

    let mut lule_configs: PathBuf = dirs::config_dir().unwrap_or_else(|| panic!("{} {}", "error:".red().bold(), "Path for configs is impossible to get"));
    lule_configs.push("lule");

    let mut lule_cache: PathBuf = dirs::cache_dir().unwrap_or_else(|| panic!("{} {}", "error:".red().bold(), "Path for configs is impossible to get"));
    lule_cache.push("lule");



    scheme.set_theme(Some("dark".to_string()));
    scheme.set_config(Some(lule_configs.to_str().unwrap().to_string()));
    scheme.set_cache(Some(lule_cache.to_str().unwrap().to_string()));
    scheme.set_palette(Some("pigment".to_string()));
}
