use crate::scheme::*;
use crossterm::{
    execute,
    terminal::{size, Clear, ClearType, ScrollUp, SetSize},
};
use std::io::stdout;

pub fn display_image(
    scheme: &Scheme,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let (cols, rows) = size()?;
    execute!(stdout(), SetSize(10, 10), ScrollUp(5))?;

    let filename = scheme.image().clone().unwrap();
    let conf = viuer::Config {
        // restore_cursor: true,
        // use_kitty: true,
        width: Some(width),
        height: Some(height),
        ..Default::default()
    };

    viuer::print_from_file(&filename, &conf)?;

    execute!(stdout(), SetSize(cols, rows))?;
    execute!(stdout(), Clear(ClearType::FromCursorDown))?;
    Ok(())
}
