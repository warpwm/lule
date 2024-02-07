use std::io::Write;

use pastel::ansi::{self, Brush, ToAnsiStyle};
use pastel::named::{NamedColor, NAMED_COLORS};
use pastel::Color;
use pastel::Format;

pub struct Canvas {
    height: usize,
    width: usize,
    pixels: Vec<Option<Color>>,
    chars: Vec<Option<char>>,
    brush: Brush,
}

impl Canvas {
    pub fn new(height: usize, width: usize, brush: Brush) -> Self {
        assert!(height % 2 == 0);

        let mut pixels = vec![];
        pixels.resize(height * width, None);
        let mut chars = vec![];
        chars.resize(height / 2 * width, None);

        Canvas {
            height,
            width,
            pixels,
            chars,
            brush,
        }
    }

    pub fn draw_rect(
        &mut self,
        row: usize,
        col: usize,
        height: usize,
        width: usize,
        color: &Color,
    ) {
        for i in 0..height {
            for j in 0..width {
                *self.pixel_mut(row + i, col + j) = Some(color.clone());
            }
        }
    }

    pub fn draw_checkerboard(
        &mut self,
        row: usize,
        col: usize,
        height: usize,
        width: usize,
        dark: &Color,
        light: &Color,
    ) {
        for i in 0..height {
            for j in 0..width {
                let color = if (i + j) % 2 == 0 { dark } else { light };
                *self.pixel_mut(row + i, col + j) = Some(color.clone());
            }
        }
    }

    pub fn draw_text(&mut self, row: usize, col: usize, text: &str) {
        assert!(row % 2 == 0);

        for (j, c) in text.chars().enumerate() {
            *self.char_mut(row / 2, col + j) = Some(c);
        }
    }

    pub fn print(&self, out: &mut dyn Write) -> Result<(), Box<dyn std::error::Error>> {
        for i_div_2 in 0..self.height / 2 {
            for j in 0..self.width {
                if let Some(c) = self.char(i_div_2, j) {
                    write!(out, "{}", c)?;
                } else {
                    let p_top = self.pixel(2 * i_div_2, j);
                    let p_bottom = self.pixel(2 * i_div_2 + 1, j);

                    match (p_top, p_bottom) {
                        (Some(top), Some(bottom)) => write!(
                            out,
                            "{}",
                            self.brush.paint("▀", top.ansi_style().on(bottom))
                        )?,
                        (Some(top), None) => write!(out, "{}", self.brush.paint("▀", top))?,
                        (None, Some(bottom)) => write!(out, "{}", self.brush.paint("▄", bottom))?,
                        (None, None) => write!(out, " ")?,
                    };
                }
            }
            writeln!(out)?;
        }
        Ok(())
    }

    fn pixel(&self, i: usize, j: usize) -> &Option<Color> {
        assert!(i < self.height);
        assert!(j < self.width);
        &self.pixels[i * self.width + j]
    }

    fn pixel_mut(&mut self, i: usize, j: usize) -> &mut Option<Color> {
        assert!(i < self.height);
        assert!(j < self.width);
        &mut self.pixels[i * self.width + j]
    }

    fn char(&self, i: usize, j: usize) -> &Option<char> {
        assert!(i < self.height / 2);
        assert!(j < self.width);
        &self.chars[i * self.width + j]
    }

    fn char_mut(&mut self, i: usize, j: usize) -> &mut Option<char> {
        assert!(i < self.height / 2);
        assert!(j < self.width);
        &mut self.chars[i * self.width + j]
    }
}

pub fn similar_colors(color: &Color) -> Vec<&NamedColor> {
    let mut colors: Vec<&NamedColor> = NAMED_COLORS.iter().collect();
    colors.sort_by_key(|nc| (1000.0 * nc.color.distance_delta_e_ciede2000(color)) as i32);
    colors.dedup_by(|n1, n2| n1.color == n2.color);
    colors
}

pub fn show_color(
    handle: &mut dyn Write,
    mode: ansi::Mode,
    color: &Color,
    id: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let checkerboard_size: usize = 16;
    let color_panel_size: usize = 12;

    let checkerboard_position_y: usize = 0;
    let checkerboard_position_x: usize = 2;
    let color_panel_position_y: usize =
        checkerboard_position_y + (checkerboard_size - color_panel_size) / 2;
    let color_panel_position_x: usize =
        checkerboard_position_x + (checkerboard_size - color_panel_size) / 2;
    let text_position_x: usize = checkerboard_size + 2 * checkerboard_position_x;
    let text_position_y: usize = 0;

    let mut canvas = Canvas::new(checkerboard_size, 51, ansi::Brush::from_mode(Some(mode)));
    canvas.draw_checkerboard(
        checkerboard_position_y,
        checkerboard_position_x,
        checkerboard_size,
        checkerboard_size,
        &Color::graytone(0.94),
        &Color::graytone(0.71),
    );
    canvas.draw_rect(
        color_panel_position_y,
        color_panel_position_x,
        color_panel_size,
        color_panel_size,
        color,
    );

    canvas.draw_text(text_position_y, text_position_x, &format!("Color: {}", id));

    #[allow(clippy::identity_op)]
    canvas.draw_text(
        text_position_y + 2,
        text_position_x,
        &format!("Hex: {}", color.to_rgb_hex_string(true)),
    );
    canvas.draw_text(
        text_position_y + 4,
        text_position_x,
        &format!("RGB: {}", color.to_rgb_string(Format::Spaces)),
    );
    canvas.draw_text(
        text_position_y + 6,
        text_position_x,
        &format!("HSL: {}", color.to_hsl_string(Format::Spaces)),
    );

    // canvas.draw_text(
    //     text_position_y + 8,
    //     text_position_x,
    //     "Most similar:",
    // );

    let similar = similar_colors(color);
    for (i, nc) in similar.iter().enumerate().take(3) {
        canvas.draw_text(text_position_y + 10 + 2 * i, text_position_x + 7, nc.name);
        canvas.draw_rect(
            text_position_y + 10 + 2 * i,
            text_position_x + 1,
            2,
            5,
            &nc.color,
        );
    }

    canvas.print(handle)?;
    writeln!(handle)?;
    Ok(())
}
