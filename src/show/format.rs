use crate::scheme::*;
use crate::show::canvas;
use colored::*;
use pastel::ansi;
use std::ops::Range;

pub fn show_colors(scheme: &Scheme, colrange: Range<usize>, padding: usize) {
    let colors = scheme.colors().clone().unwrap();
    for i in colrange {
        let val = if true {
            format!("  {:#03}  ", i)
        } else {
            format!(
                "{}{}{}",
                " ".repeat(padding),
                colors[i].to_rgb_hex_string(true),
                " ".repeat(padding)
            )
        };
        if (i % 12 == 4 && i > 16) || (i == 16 || i == 8) {
            println!()
        };
        if i == 16 || i == 232 {
            println!()
        };
        print!(
            "{}",
            val.on_truecolor(
                colors[i].to_rgba().r,
                colors[i].to_rgba().g,
                colors[i].to_rgba().b
            )
            .color(if colors[i].to_lab().l < 30.0 {
                "white"
            } else {
                "black"
            })
        );
    }
    println!();
}

pub fn show_pastel_colors(scheme: &Scheme, colrange: Range<usize>) {
    let stdout = std::io::stdout();
    let mut stdout_lock_handle = stdout.lock();

    for i in colrange {
        canvas::show_color(
            &mut stdout_lock_handle,
            ansi::Mode::TrueColor,
            &scheme.colors().clone().unwrap()[i],
            i,
        )
        .ok();
    }
}

pub fn show_specified_colors(colors: Vec<pastel::Color>, padding: usize) {
    for (i, color) in colors.iter().enumerate() {
        let val = format!(
            "{}{}{}",
            " ".repeat(padding),
            color.to_rgb_hex_string(true),
            " ".repeat(padding)
        );
        if (i % 12 == 4 && i > 16) || (i == 16 || i == 8) {
            println!()
        };
        print!(
            "{}",
            val.on_truecolor(color.to_rgba().r, color.to_rgba().g, color.to_rgba().b)
                .color(if color.to_lab().l < 30.0 {
                    "white"
                } else {
                    "black"
                })
        );
    }
}
