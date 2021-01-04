use anyhow::Result;
use std::path::PathBuf;
use crate::var;
use crate::gen::palette;
use crate::show::format;
use crate::show::viuwer;
use crate::scheme::*;
use crate::helper::*;
use crate::cli::create;

pub fn run(app: &clap::ArgMatches, output: &mut WRITE, scheme: &mut SCHEME) -> Result<()> {
    let sub = app.subcommand_matches("colors").unwrap();
    var::defs::concatinate(scheme);
    var::envi::concatinate(scheme);
    var::args::concatinate(app, scheme);
    var::pipe::concatinate(scheme);


    scheme.set_script(None);
    if sub.is_present("gen") {
        create::new_palette(output, scheme)?;
    }


    if let Some(cachepath) = scheme.cache().clone() {
        let mut color_temp = PathBuf::from(&cachepath);
        color_temp.push("colors");
        if let Ok(content) = palette::colors_from_file(color_temp) {
            output.set_colors(content);
        }

        let mut wall_temp = PathBuf::from(&cachepath);
        wall_temp.push("wallpaper");
        if let Ok(content) = file_to_string(wall_temp) {
            output.set_wallpaper(content);
        }

        let mut theme_temp = PathBuf::from(&cachepath);
        theme_temp.push("theme");
        if let Ok(content) = file_to_string(theme_temp) {
            output.set_theme(content);
        }
    }



    let (cols, rows) = crossterm::terminal::size().ok().unwrap();
    if let Some(arg) = sub.value_of("action") {
        // let values = write::get_json(output);
        if atty::isnt(atty::Stream::Stdout) {
            for color in output.colors().iter() {
                println!("{}", color.to_rgb_hex_string(true));
            }
        } else {
            if arg ==  "image" {
                viuwer::display_image(&output, (cols).into(), (rows -1).into()).ok();
            } else if arg ==  "ansii" {
                format::show_colors(&output, 0..output.colors().len(), 4);
            } else if arg ==  "list" {
                format::show_pastel_colors(&output, 0..output.colors().len());
            } else if arg ==  "mix" {
                println!("Wallpaper:");
                viuwer::display_image(&output, (cols).into(), (rows -3).into()).ok();
                println!("Colors:");
                format::show_colors(&output, 0..16, ((cols - 56) / 16).into());
            }
        }
    }
    Ok(())
}
