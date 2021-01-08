pub mod temp;
pub mod defs;
pub mod envi;
pub mod pipe;
pub mod args;
pub mod file;

use clap;
use crate::scheme::*;
use colored::*;

pub fn concatinate(app: &clap::ArgMatches, scheme: &mut SCHEME) {
    temp::concatinate(scheme);
    defs::concatinate(scheme);
    envi::concatinate(scheme);
    args::concatinate(app, scheme);
    pipe::concatinate(scheme);

    if let Some(s) = scheme.scripts() {
        let mut scripts = s.to_vec();
        scripts.dedup();
        scheme.set_scripts(Some(scripts));
    }

    if scheme.image().is_none() && scheme.walldir().is_none() {
        eprintln!("{} {} {} {}", "error:".red().bold(), "Environment variable", "'$LULE_W'".yellow(), "is empty");
        eprintln!("{} {} {} {}", "error:".red().bold(), "Argument option", "'--wallpath'".yellow(), "is not set");
        eprintln!("{} {} {} {}", "error:".red().bold(), "Image argument", "'--image'".yellow(), "is not given");
        eprintln!("\n{}\n\t{}\n\n{} {}", "USAGE".yellow(), "lule help <subcommands>...", 
            "For more information try", "--help".blue() );
        std::process::exit(1);
    }
}
