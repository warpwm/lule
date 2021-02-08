mod fun;
mod cli;
mod var;
mod gen;
mod show;
mod scheme;

extern crate file;
extern crate serde;
extern crate serde_json;
extern crate rayon;

#[macro_use]
extern crate serde_derive;

use std::env;
use scheme::*;


fn main() {
    let mut scheme = SCHEME::init();

    let show_logo = if env::args().len() > 1 { false } else { true };

    let app = cli::build_cli(show_logo).get_matches();
    // var::concatinate(&app, &mut scheme);

    if let Some(subcommand) = app.subcommand_name() {
        match subcommand {
            "colors" => cli::colors::run(&app, &mut scheme),
            "create" => cli::create::run(&app, &mut scheme),
            "config" => cli::config::run(&app, &mut scheme),
            "daemon" => cli::daemon::run(&app, &mut scheme),
            "test" => cli::test::run(&app, &mut scheme),
            _ => Ok(())
        }.ok();
    }
}
