use crate::fun::text;
use crate::scheme::*;
use anyhow::{Context, Result};

pub fn concatinate(scheme: &mut Scheme) {
    let lule_scheme = text::pather(vec!["lule_scheme"], std::env::temp_dir());
    if let Ok(scheme_string) = text::file_to_string(lule_scheme) {
        if let Ok(sh) = make_scheme(scheme_string) {
            *scheme = sh;
        }
    }
    scheme.set_image(None);
}

fn make_scheme(data: String) -> Result<Scheme> {
    let scheme: Scheme =
        serde_json::from_str(&data).context("something got fucked-up reaading json")?;
    Ok(scheme)
}
