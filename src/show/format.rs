use crate::show::canvas;
use crate::scheme::*;
use colored::*;
use pastel::ansi;
use atty::Stream;
use std::ops::Range;

pub fn show_colors(output: &WRITE, colrange: Range::<usize>, padding: usize) {
    for i in colrange {
        let val = if !true { format!("  {:#03}  ", i) } else { format!("{}{}{}",
            " ".repeat(padding),
            output.colors()[i].to_rgb_hex_string(true), 
            " ".repeat(padding))
        };
        if (i % 12 == 4 && i > 16) || (i == 16 || i == 8) { println!() };
        if i == 16 || i == 232 { println!() };
        print!("{}",
            val.on_truecolor(
                output.colors()[i].to_rgba().r.into(),
                output.colors()[i].to_rgba().g.into(), 
                output.colors()[i].to_rgba().b.into()
            ).color( if output.colors()[i].to_lab().l < 50.0 { "white" } else { "black" } )
            );
    }
    println!();
}

pub fn show_pastel_colors(output: &WRITE, colrange: Range::<usize>) {
    let stdout = std::io::stdout();
    let mut stdout_lock_handle = stdout.lock();

    for i in colrange {
        if atty::is(Stream::Stdout) {
            canvas::show_color(&mut stdout_lock_handle, ansi::Mode::TrueColor, &output.colors()[i], i).ok();
        } else {
            &output.colors()[i].to_rgb_hex_string(true);
        }
    }
}