use crate::scheme::*;
use anyhow::{Context, Result};
use std::io::{self, BufRead};

pub fn concatinate(scheme: &mut Scheme) {
    if atty::isnt(atty::Stream::Stdin) {
        if let Ok(input) = read_stdin() {
            if let Ok(sh) = make_scheme(input) {
                *scheme = sh;
            }
        }
    }
}

fn read_stdin() -> Result<String> {
    let mut input = String::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        input.push_str(&line.unwrap());
    }
    Ok(input)
}

fn make_scheme(data: String) -> Result<Scheme> {
    let scheme: Scheme =
        serde_json::from_str(&data).context("something got fucked-up reaading json")?;
    Ok(scheme)
}
