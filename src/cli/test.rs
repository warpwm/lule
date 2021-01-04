use crate::scheme::*;
use anyhow::Result;
use std::env;

use crate::var;
use crate::gen::palette;
use crate::gen::generate;
use crate::gen::templ;
use crate::helper;
use crate::show::viuwer;
use crate::show::format;

pub fn run(app: &clap::ArgMatches, output: &mut WRITE, scheme: &mut SCHEME) -> Result<()> {
    test_colors(app, output, scheme)?;
    let mut original = env::temp_dir(); original.push("lule_original");
    let mut replaced = env::temp_dir(); replaced.push("lule_replaced");
    templ::generate_template(original, replaced, output)?;

    Ok(())
}


fn test_colors(app: &clap::ArgMatches, output: &mut WRITE, scheme: &mut SCHEME) -> Result<()> {
    // let mut pipe_name = std::env::temp_dir();
    // pipe_name.push("lule_pipe");

    var::defs::concatinate(scheme);
    var::envi::concatinate(scheme);
    var::args::concatinate(app, scheme);
    var::pipe::concatinate(scheme);


    let wallpaper = scheme.walldir().clone().unwrap();
    if scheme.image().is_none() {
        scheme.set_image(Some(helper::random_image(&wallpaper)));
    }

    let palette = palette::palette_from_image(scheme.image().clone().unwrap());
    scheme.set_colors(Some(palette.clone()));

    output.set_theme(scheme.theme().clone().unwrap());
    output.set_colors(generate::get_all_colors(scheme));
    output.set_wallpaper(scheme.image().clone().unwrap());



    let (cols, rows) = crossterm::terminal::size().ok().unwrap();
    viuwer::display_image(&output, (cols-10).into(), (rows -13).into()).ok();
    println!("Palette");
    let colors: Vec<pastel::Color> = palette.into_iter().map(|x| pastel::Color::from_hex(&x)).collect();
    format::show_specified_colors(colors.clone(), ((cols - 56) / 16).into());
    println!("\n6th");
    format::show_specified_colors(generate::gen_main_six(&colors), ((cols - 56) / 16).into());
    println!("\nColors");
    format::show_colors(&output, 0..16, ((cols - 56) / 16).into());
    Ok(())
}
