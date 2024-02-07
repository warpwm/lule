#![allow(dead_code)]
#![allow(unused_must_use)]

use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};

use colored::*;
use rand::seq::IteratorRandom;

pub fn vaid_image(path: &str) -> String {
    match image::open(path) {
        Ok(_) => path.to_owned(),
        Err(_) => {
            eprintln!(
                "{} Path {} is not a valid image file",
                "error:".red().bold(),
                path.yellow()
            );
            std::process::exit(1);
        }
    }
}

// TODO: check if folder is empty, is valid, exists or has other files than images
pub fn random_image(path: &str) -> String {
    let mut rng = rand::thread_rng();
    let files = fs::read_dir(path).unwrap();
    let file = files.choose(&mut rng).unwrap().unwrap();
    let filepath = file.path().display().to_string();
    vaid_image(&filepath)
}

pub fn write_to_file(filename: PathBuf, content: &[u8]) {
    let mut file_name = File::create(filename.clone()).unwrap_or_else(|err| {
        eprintln!(
            "{} Could not create file {} -> {}",
            "error:".red().bold(),
            filename.as_os_str().to_str().unwrap().yellow(),
            err
        );
        std::process::exit(1);
    });

    file_name.write_all(content).unwrap_or_else(|err| {
        eprintln!(
            "{} Could not write into {} -> {}",
            "error:".red().bold(),
            filename.as_os_str().to_str().unwrap().yellow(),
            err
        );
        std::process::exit(1);
    });
}

pub fn write_temp_file(filename: &str, content: &[u8]) {
    let mut file_name = env::temp_dir();
    file_name.push(filename);
    write_to_file(file_name, content);
}

pub fn pather(dirs: Vec<&str>, path: PathBuf) -> PathBuf {
    let mut new_path = path.clone();
    for s in dirs {
        new_path.push(s);
    }
    new_path
}

pub fn copy_to(dir1: PathBuf, dir2: PathBuf) {
    fs::copy(dir1.to_str().unwrap(), dir2.to_str().unwrap());
}

pub fn lines_to_vec(filename: PathBuf) -> Vec<String> {
    // File must exist in current path before this produces output
    let mut content = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            content.push(line)
        }
    }
    content
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn file_to_string(filename: PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let string = file::get_text(filename.to_str().unwrap())?;
    Ok(string)
}
