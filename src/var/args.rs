use clap;
use crate::scheme::*;
use crate::helper;

pub fn concatinate(app: &clap::ArgMatches, scheme: &mut SCHEME) {
    if let Some(sub) = app.subcommand_matches("create"){
        if let Some(arg) = sub.value_of("image") {
            scheme.set_image(Some(helper::vaid_image(arg)));
        } else if let Some(arg) = sub.value_of("wallpath") {
            scheme.set_image(Some(helper::random_image(arg)));
        }
        if let Some(arg) = sub.value_of("theme") {
            scheme.set_theme(Some(arg.to_string()));
        }
    };
    if let Some(sub) = app.subcommand_matches("config"){
        if let Some(arg) = sub.value_of("theme") {
            scheme.set_theme(Some(arg.to_string()));
        }
    };
    if let Some(sub) = app.subcommand_matches("daemon"){
        if let Some(arg) = sub.value_of("loop") {
            let value = arg.parse::<usize>().expect("--loop value must be a number");
            scheme.set_looop(Some(value));
        } else {
            scheme.set_looop(Some(300));
        }
    };
    if let Some(sub) = app.subcommand_matches("test"){
        if let Some(arg) = sub.value_of("image") {
            scheme.set_image(Some(helper::vaid_image(arg)));
        }
        if let Some(arg) = sub.value_of("theme") {
            scheme.set_theme(Some(arg.to_string()));
        }
    };
}
