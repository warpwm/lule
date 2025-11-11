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

    let show_logo = env::args().len() <= 1 && atty::is(atty::Stream::Stdout);
    let logo = if show_logo {
        // Try to read logo from executable's directory first, fallback to relative path
        let exe_dir = env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()));
        
        let logo_path = exe_dir
            .as_ref()
            .map(|p| p.join("../resources/logo.txt"))
            .and_then(|p| std::fs::read_to_string(p).ok())
            .or_else(|| std::fs::read_to_string("resources/logo.txt").ok())
            .unwrap_or(String::new());
        
        logo_path
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
