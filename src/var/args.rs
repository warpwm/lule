use crate::fun::text;
use crate::scheme::*;
use clap;

pub fn concatinate(app: &clap::ArgMatches, scheme: &mut Scheme) {
    if app.values_of("script").is_some() {
        let vals: Vec<&str> = app.values_of("script").unwrap().collect();
        let mut scripts = Vec::new();
        if let Some(s) = scheme.scripts() {
            scripts = s.to_vec();
        }
        for val in vals {
            scripts.push(val.to_string())
        }
        scheme.set_scripts(Some(scripts));
    }

    if app.values_of("pattern").is_some() {
        let vals: Vec<&str> = app.values_of("pattern").unwrap().collect();
        let mut patterns = Vec::new();
        for val in vals {
            let s: Vec<&str> = val.split_terminator(':').collect();
            // TODO: better error
            if s.len() == 2 {
                patterns.push((s[0].to_string(), s[1].to_string()));
            }
        }
        scheme.set_patterns(Some(patterns));
    }

    if let Some(sub) = app.subcommand_matches("create") {
        if let Some(arg) = sub.value_of("image") {
            scheme.set_image(Some(text::vaid_image(arg)));
        } else if let Some(arg) = sub.value_of("wallpath") {
            scheme.set_image(Some(text::random_image(arg)));
        }
        if let Some(arg) = sub.value_of("theme") {
            scheme.set_theme(Some(arg.to_string()));
        }
    };
    if let Some(sub) = app.subcommand_matches("config") {
        if let Some(arg) = sub.value_of("theme") {
            scheme.set_theme(Some(arg.to_string()));
        }
    };
    if let Some(sub) = app.subcommand_matches("daemon") {
        if let Some(arg) = sub.value_of("loop") {
            let value = arg.parse::<usize>().expect("--loop value must be a number");
            scheme.set_looop(Some(value));
        } else {
            scheme.set_looop(Some(300));
        }
    };
    if let Some(sub) = app.subcommand_matches("test") {
        if let Some(arg) = sub.value_of("image") {
            scheme.set_image(Some(text::vaid_image(arg)));
        }
        if let Some(arg) = sub.value_of("theme") {
            scheme.set_theme(Some(arg.to_string()));
        }
    };
}
