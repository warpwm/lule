use crate::show::canvas;
use crate::scheme::*;
use colored::*;
use pastel::ansi;
use std::ops::Range;

pub fn show_colors(scheme: &SCHEME, colrange: Range::<usize>, padding: usize) {
    let colors = scheme.colors().clone().unwrap();
    for i in colrange {
        let val = if true { format!("  {:#03}  ", i) } else { format!("{}{}{}",
            " ".repeat(padding),
            colors[i].to_rgb_hex_string(true), 
            " ".repeat(padding))
        };
        if (i % 12 == 4 && i > 16) || (i == 16 || i == 8) { println!() };
        if i == 16 || i == 232 { println!() };
        print!("{}",
            val.on_truecolor(
                colors[i].to_rgba().r.into(),
                colors[i].to_rgba().g.into(), 
                colors[i].to_rgba().b.into()
            ).color( if colors[i].to_lab().l < 30.0 { "white" } else { "black" } )
            );
    }
    println!();
}

pub fn show_pastel_colors(scheme: &SCHEME, colrange: Range::<usize>) {
    let stdout = std::io::stdout();
    let mut stdout_lock_handle = stdout.lock();

    for i in colrange {
        canvas::show_color(&mut stdout_lock_handle, ansi::Mode::TrueColor, &scheme.colors().clone().unwrap()[i], i).ok();
    }
}

pub fn show_specified_colors(colors: Vec<pastel::Color>, padding: usize) {
    for i in 0..colors.len() {
        let val = format!("{}{}{}",
            " ".repeat(padding),
            colors[i].to_rgb_hex_string(true), 
            " ".repeat(padding));
        if (i % 12 == 4 && i > 16) || (i == 16 || i == 8) { println!() };
        print!("{}",
            val.on_truecolor(
                colors[i].to_rgba().r.into(),
                colors[i].to_rgba().g.into(), 
                colors[i].to_rgba().b.into()
            ).color( if colors[i].to_lab().l < 30.0 { "white" } else { "black" } )
        );
    }
}
