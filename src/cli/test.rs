use crate::scheme::*;
use anyhow::Result;
use crate::var;
use crate::gen::palette;
use crate::gen::generate;
use crate::helper;
use crate::show::viuwer;
use crate::show::format;

pub fn run(app: &clap::ArgMatches, output: &mut WRITE, scheme: &mut SCHEME) -> Result<()> {
    test_colors(app, output, scheme)
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
    let mut colors = Vec::new();
    for color in palette {
        colors.push(pastel::Color::from_hex(&color));
    }
    format::show_specified_colors(colors.clone(), ((cols - 56) / 16).into());
    println!("\n6th");
    format::show_specified_colors(generate::gen_main_six(&colors), ((cols - 56) / 16).into());
    println!("\nColors");
    format::show_colors(&output, 0..16, ((cols - 56) / 16).into());
    Ok(())
}
