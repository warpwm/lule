pub mod temp;
pub mod defs;
pub mod envi;
pub mod pipe;
pub mod args;
pub mod file;


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
        eprintln!("{} Environment variable {} is empty", "error:".red().bold(), "'$LULE_W'".yellow());
        eprintln!("{} Argument option {} is not set", "error:".red().bold(), "'--wallpath'".yellow());
        eprintln!("{} Image argument {} is not given", "error:".red().bold(), "'--image'".yellow());
        eprintln!("\n{}\n\tlule help <subcommands>...\n\nFor more information try {}", "USAGE".yellow(), "--help".blue() );
        std::process::exit(1);
    }
}
