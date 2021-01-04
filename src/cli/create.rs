use crate::gen::generate;
use crate::gen::palette;
use crate::gen::write;
use crate::gen::execute;
use crate::var;
use crate::scheme::*;
use crate::helper;
use anyhow::Result;

pub fn run(app: &clap::ArgMatches, output: &mut WRITE, scheme: &mut SCHEME) -> Result<()> {
    let sub = app.subcommand_matches("create").unwrap();
    var::defs::concatinate(scheme);
    var::envi::concatinate(scheme);
    var::args::concatinate(app, scheme);
    var::pipe::concatinate(scheme);

    if atty::isnt(atty::Stream::Stdout) {
        // write::write_temp(&output);
        // println!("{}", &values);
    } else {
        if let Some(arg) = sub.value_of("action") {
            if arg ==  "set" {
                new_palette(output, scheme)?;
            }
            // if arg ==  "regen" {
            //     generate::get_all_colors(output, scheme);
            //     write::write_temp_colors(&output);
            //     write::write_cache_colors(scheme, values);
            //     write::copy_to_cache(scheme);
            //     execute::external_command();
            // }
        }
    }
    Ok(())
}

pub fn new_palette(output: &mut WRITE, scheme: &mut SCHEME) -> Result<()> {
    let wallpaper = scheme.walldir().clone().unwrap();
    scheme.set_image(Some(helper::random_image(&wallpaper)));
    println!("{}", &scheme.image().clone().unwrap());

    let palette: Vec<String>;
    if let Some(content) = scheme.palette() {
        match content.as_str() {
            "pigment" => {
                palette = palette::palette_from_image(scheme.image().clone().unwrap());
                println!("{}", &scheme.image().clone().unwrap());
                helper::write_temp_file("lule_palette", palette.join("\n").as_bytes());
                scheme.set_colors(Some(palette));
            },
            _ => unreachable!(),
        };
    }

    output.set_theme(scheme.theme().clone().unwrap());
    output.set_colors(generate::get_all_colors(scheme));
    output.set_wallpaper(scheme.image().clone().unwrap());

    let values = write::output_to_json(output, false);
    write::write_temp(&output);
    write::write_cache(&scheme);
    write::write_cache_json(scheme, values);
    execute::external_command();
    Ok(())
}


pub fn old_palette(output: &mut WRITE, scheme: &mut SCHEME) -> Result<()> {
    let mut lule_palette = std::env::temp_dir(); lule_palette.push("lule_palette");
    scheme.set_colors(Some(helper::lines_to_vec(lule_palette)));

    output.set_theme(scheme.theme().clone().unwrap());
    output.set_colors(generate::get_all_colors(scheme));

    let mut lule_wallpaper = std::env::temp_dir(); lule_wallpaper.push("lule_wallpaper");
    output.set_wallpaper(helper::file_to_string(lule_wallpaper).unwrap());

    let values = write::output_to_json(output, false);
    write::write_temp(&output);
    write::write_cache(&scheme);
    write::write_cache_json(scheme, values);
    execute::external_command();
    Ok(())
}
