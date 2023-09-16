mod cli;
mod fun;
mod gen;
mod scheme;
mod show;
mod var;

extern crate file;
extern crate rayon;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use scheme::*;
use std::env;

fn main() {
    let mut scheme = Scheme::init();

    let show_logo = env::args().len() <= 1;

    let app = cli::build_cli(show_logo).get_matches();
    // var::concatinate(&app, &mut scheme);

    if let Some(subcommand) = app.subcommand_name() {
        match subcommand {
            "colors" => cli::colors::run(&app, &mut scheme),
            "create" => cli::create::run(&app, &mut scheme),
            "config" => cli::config::run(&app, &mut scheme),
            "daemon" => cli::daemon::run(&app, &mut scheme),
            "test" => cli::test::run(&app, &mut scheme),
            _ => Ok(()),
        }
        .ok();
    }
}
