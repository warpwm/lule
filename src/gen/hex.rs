use core::fmt;

use nom::{character::complete::*, combinator::opt, error::ErrorKind, Err, IResult};
use pastel::{Color, RGBA};

#[derive(Debug, Clone, PartialEq)]
pub struct Hex {
    pub val: String,
}

impl From<&Color> for Hex {
    fn from(color: &Color) -> Self {
        let rgb = RGBA::<u8>::from(color);
        Hex {
            val: format!("{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b),
        }
    }
}

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.val)
    }
}

fn hex_to_u8_unsafe(num: &str) -> u8 {
    u8::from_str_radix(num, 16).unwrap()
}

fn opt_hash_char(s: &str) -> IResult<&str, Option<char>> {
    opt(char('#'))(s)
}

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgb(r, g, b)
}

pub fn parse_hex(input: &str) -> IResult<&str, Color> {
    let (input, _) = opt_hash_char(input)?;
    let (input, hex_chars) = hex_digit1(input)?;
    match hex_chars.len() {
        6 => {
            let r = hex_to_u8_unsafe(&hex_chars[0..2]);
            let g = hex_to_u8_unsafe(&hex_chars[2..4]);
            let b = hex_to_u8_unsafe(&hex_chars[4..6]);
            Ok((input, rgb(r, g, b)))
        }
        3 => {
            let r = hex_to_u8_unsafe(&hex_chars[0..1]);
            let g = hex_to_u8_unsafe(&hex_chars[1..2]);
            let b = hex_to_u8_unsafe(&hex_chars[2..3]);
            let r = r * 16 + r;
            let g = g * 16 + g;
            let b = b * 16 + b;
            Ok((input, rgb(r, g, b)))
        }
        _ => Err(Err::Error((
            "Expected hex string of 3 or 6 characters length",
            ErrorKind::Many1,
        ))),
    }
}

pub fn color_from_hex(hex_string: &str) -> Color {
    parse_hex(hex_string).unwrap().1
}

#[allow(unused)]
pub fn color_to_rgb_hex_string(color: &Color, leading_hash: bool) -> String {
    let hex = Hex::from(color);
    format!("{}{}", if leading_hash { "#" } else { "" }, hex.val)
}
