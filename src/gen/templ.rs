use anyhow::Result;
use std::path::PathBuf;
use crate::helper;
use crate::scheme::*;
use templar;
use templar::*;

pub fn generate_template(original: PathBuf, _replaced: PathBuf, output: &WRITE) -> Result<()> {

    let mut content = String::new();
    if let Ok(cont) = helper::file_to_string(original) {
        content = cont;
    }

    if let Err(e) = templar::Templar::global().parse(&content) {
        println!("{}", e);
    };

    let template = templar::Templar::global().parse(&content)?;
    println!("{}", content);
    let mut data: templar::Document = templar::Document::default();
    for (i, color) in output.colors().iter().enumerate() {
        let name = "color".to_string() + &i.to_string();
        data[name] = color.to_rgb_hex_string(true).into();
    }
    data["background"] = output.colors()[0].to_rgb_hex_string(true).into();
    data["foreground"] = output.colors()[15].to_rgb_hex_string(true).into();
    data["cursor"] = output.colors()[1].to_rgb_hex_string(true).into();
    data["accent"] = output.colors()[1].to_rgb_hex_string(true).into();

    let context = templar::StandardContext::new();
    context.set(data)?;

    println!("{}", template.render(&context)?);


    Ok(())
}
