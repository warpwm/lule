use anyhow::Result;
use std::path::PathBuf;
use crate::var;
use crate::gen::palette;
use crate::show::format;
use crate::show::viuwer;
use crate::scheme::*;
use crate::fun::text;
use crate::gen::apply;

pub fn run(app: &clap::ArgMatches, scheme: &mut SCHEME) -> Result<()> {
    let sub = app.subcommand_matches("colors").unwrap();
    var::concatinate(app, scheme);


    scheme.set_scripts(None);
    if sub.is_present("gen") {
        apply::write_colors(scheme, false)?;
    }


    if let Some(cachepath) = scheme.cache().clone() {
        let mut color_temp = PathBuf::from(&cachepath);
        color_temp.push("colors");
        if let Ok(content) = palette::colors_from_file(color_temp) {
            scheme.set_colors(Some(content));
        }

        let mut wall_temp = PathBuf::from(&cachepath);
        wall_temp.push("wallpaper");
        if let Ok(content) = text::file_to_string(wall_temp) {
            scheme.set_image(Some(content));
        }

        let mut theme_temp = PathBuf::from(&cachepath);
        theme_temp.push("theme");
        if let Ok(content) = text::file_to_string(theme_temp) {
            scheme.set_theme(Some(content));
        }
    }



    let (cols, rows) = crossterm::terminal::size().ok().unwrap();
    if let Some(arg) = sub.value_of("action") {
        // let values = write::get_json(output);
        if atty::isnt(atty::Stream::Stdout) {
            for color in scheme.colors().clone().unwrap().iter() {
                println!("{}", color.to_rgb_hex_string(true));
            }
        } else {
            if arg ==  "image" {
                viuwer::display_image(&scheme, (cols).into(), (rows -1).into()).ok();
            } else if arg ==  "ansii" {
                format::show_colors(&scheme, 0..256, 4);
            } else if arg ==  "list" {
                format::show_pastel_colors(&scheme, 0..256);
            } else if arg ==  "mix" {
                viuwer::display_image(&scheme, (cols).into(), (rows -3).into()).ok();
                println!("Wallpaper: {}, \t\t Colors: 1-16", scheme.image().clone().unwrap());
                format::show_colors(&scheme, 0..16, ((cols - 56) / 16).into());
            }
        }
    }
    Ok(())
}
