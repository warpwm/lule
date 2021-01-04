use anyhow::Result;
use std::path::PathBuf;
use crate::helper;
use crate::scheme::*;
use templar;
use templar::*;

pub fn generate_template(original: PathBuf, replaced: PathBuf, output: &WRITE) -> Result<()> {

    let mut content = String::new();
    if let Ok(cont) = helper::file_to_string(original) {
        content = cont;
    }

    // if let Err(e) = templar::Templar::global().parse(&content) {
    //     println!("{}", e);
    // };

    let template = templar::Templar::global().parse(&content)?;
    let mut data: templar::Document = templar::Document::default();
    for (i, color) in output.colors().iter().enumerate() {
        let name = "color".to_string() + &i.to_string();
        data[name] = color.to_rgb_hex_string(false).into();
    }
    data["background"] = output.colors()[0].to_rgb_hex_string(false).into();
    data["foreground"] = output.colors()[15].to_rgb_hex_string(false).into();
    data["cursor"] = output.colors()[1].to_rgb_hex_string(false).into();
    data["accent"] = output.colors()[1].to_rgb_hex_string(false).into();

    data["wallpaper"] = output.wallpaper().into();
    data["theme"] = output.theme().into();

    let context = templar::StandardContext::new();
    context.set(data)?;

    let new_content = format!("{}", template.render(&context)?);
    helper::write_to_file(replaced, new_content.as_bytes());
    Ok(())
}
