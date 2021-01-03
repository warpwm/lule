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

    let palette: Vec<String>;
    if let Some(content) = scheme.palette() {
        match content.as_str() {
            "pigment" => {
                palette = palette::palette_from_image(scheme.image().clone().unwrap());
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
    if atty::isnt(atty::Stream::Stdout) {
        write::write_temp(&output);
        println!("{}", &values);
    } else {
        if let Some(arg) = sub.value_of("action") {
            if arg ==  "json" {
                write::write_temp(&output);
                println!("{}", &values);
            }
            if arg ==  "set" {
                write::write_temp(&output);
                write::write_cache(&scheme);
                write::write_cache_json(scheme, values);
                execute::external_command();
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


// fn create_colors(app: &clap::ArgMatches, output: &mut WRITE, scheme: &mut SCHEME) -> Result<()> {
//     let sub = app.subcommand_matches("create").unwrap();
//     if let Some(arg) = sub.value_of("palette") {
//         match arg.as_ref() {
//             "pigment" => palette::palette_pigment(scheme),
//             _ => println!("something went crazy wrong"),
//         }
//     };
//     generate::get_all_colors(output, scheme);
//     Ok(())
// }
