#![allow(dead_code)]

extern crate getset;
use getset::{CopyGetters, Getters, MutGetters, Setters};
use std::collections::HashMap as Map;

#[derive(Serialize, Deserialize, Debug, Clone, CopyGetters, Getters, MutGetters, Setters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
pub struct SCHEME {
    #[serde(skip)]
    colors: Option<Vec<pastel::Color>>,
    image: Option<String>,
    theme: Option<String>,
    pigments: Option<Vec<String>>,
    scheme: Option<String>,
    walldir: Option<String>,
    #[serde(skip)]
    config: Option<String>,
    #[serde(skip)]
    cache: Option<String>,
    scripts: Option<Vec<String>>,
    patterns: Option<Vec<(String, String)>>,
    looop: Option<usize>,
    palette: Option<String>,
    sort: Option<String>,
    saturation: Option<f32>,
    illumination: Option<f32>,
    hue: Option<f32>,
    difference: Option<f32>,
    blend: Option<f32>,
    mixes: Option<Map<usize, String>>,
}

impl SCHEME {
    pub fn init() -> Self {
        Self {
            colors: None,
            pigments: None,
            image: None,
            scheme: None,
            walldir: None,
            config: None,
            cache: None,
            scripts: None,
            patterns: None,
            looop: None,
            theme: None,
            palette: None,
            sort: None,
            saturation: None,
            illumination: None,
            hue: None,
            difference: None,
            blend: None,
            mixes: None 
        }
    }
    pub fn modi(&mut self, new: &SCHEME) -> &Self {
        if let Some(value) = new.colors() { self.colors = Some(value.clone()); }
        if let Some(value) = new.pigments() { self.pigments = Some(value.clone()); }
        if let Some(value) = new.image() { self.image = Some(value.clone()); }
        if let Some(value) = new.scheme() { self.scheme = Some(value.clone()); }
        if let Some(value) = new.walldir() { self.walldir = Some(value.clone()); }
        if let Some(value) = new.config() { self.config = Some(value.clone()); }
        if let Some(value) = new.cache() { self.cache = Some(value.clone()); }
        if let Some(value) = new.scripts() { self.scripts = Some(value.clone()); }
        if let Some(value) = new.patterns() { self.patterns = Some(value.clone()); }
        if let Some(value) = new.theme() { self.theme = Some(value.clone()); }
        if let Some(value) = new.palette() { self.palette = Some(value.clone()); }
        if let Some(value) = new.sort() { self.sort = Some(value.clone()); }
        if let Some(value) = new.saturation() { self.saturation = Some(value.clone()); }
        if let Some(value) = new.illumination() { self.illumination = Some(value.clone()); }
        if let Some(value) = new.hue() { self.hue = Some(value.clone()); }
        if let Some(value) = new.difference() { self.difference = Some(value.clone()); }
        if let Some(value) = new.blend() { self.blend = Some(value.clone()); }
        if let Some(value) = new.mixes() { self.mixes = Some(value.clone()); }
        self
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, CopyGetters, Getters, MutGetters, Setters)]
pub struct Special {
    pub background: String,
    pub foreground: String,
    pub cursor: String
}

#[derive(Serialize, Deserialize, Debug, Clone, CopyGetters, Getters, MutGetters, Setters)]
pub struct ProfileMap {
    pub wallpaper: String,
    pub theme: String,
    pub special: Special,
    pub colors: Map<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, CopyGetters, Getters, MutGetters, Setters)]
pub struct ProfileVec {
    pub wallpaper: String,
    pub theme: String,
    pub special: Special,
    pub colors: Vec<String>,
}
