use crate::scheme::*;
use anyhow::Result;

use crate::var;
use crate::gen::palette;
use crate::gen::generate;
use crate::gen::templ;
use crate::fun::text;
use crate::show::viuwer;
use crate::show::format;

pub fn run(app: &clap::ArgMatches, scheme: &mut SCHEME) -> Result<()> {
    test_colors(app, scheme)?;

    templ::pattern_gneration(scheme)?;

    Ok(())
}


fn test_colors(app: &clap::ArgMatches, scheme: &mut SCHEME) -> Result<()> {
    // let mut pipe_name = std::env::temp_dir();
    // pipe_name.push("lule_pipe");

    var::defs::concatinate(scheme);
    var::envi::concatinate(scheme);
    var::args::concatinate(app, scheme);
    var::pipe::concatinate(scheme);


    let wallpaper = scheme.walldir().clone().unwrap();
    if scheme.image().is_none() {
        scheme.set_image(Some(text::random_image(&wallpaper)));
    }

    let palette = palette::palette_from_image(scheme.image().clone().unwrap());
    scheme.set_pigments(Some(palette.clone()));

    let allcolors = generate::get_all_colors(scheme);
    scheme.set_colors(Some(allcolors));

    let (cols, rows) = crossterm::terminal::size().ok().unwrap();
    viuwer::display_image(&scheme, (cols-10).into(), (rows -13).into()).ok();
    println!("Palette");
    let colors: Vec<pastel::Color> = palette.into_iter().map(|x| pastel::Color::from_hex(&x)).collect();
    format::show_specified_colors(colors.clone(), ((cols - 56) / 16).into());
    println!("\n6th");
    format::show_specified_colors(generate::gen_main_six(&colors), ((cols - 56) / 16).into());
    println!("\nColors");
    format::show_colors(&scheme, 0..16, ((cols - 56) / 16).into());
    Ok(())
}
