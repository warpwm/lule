mod cli;
mod fun;
mod generator;
mod scheme;
mod show;
mod var;






use scheme::*;
use std::env;

fn main() {
    let mut scheme = Scheme::init();

    let show_logo = env::args().len() <= 1;
    let logo = if show_logo {
        std::fs::read_to_string("resources/logo.txt").unwrap_or(String::new())
    } else {
        String::new()
    };

    let app = cli::build_cli(logo.as_str()).get_matches();
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
