use clap;
use crate::scheme::*;
use crate::helper;
use std::fs;

pub fn concatinate(app: &clap::ArgMatches, scheme: &mut SCHEME) {

    if let Some(_) = app.values_of("script") {
        let vals: Vec<&str> = app.values_of("script").unwrap().collect();
        let mut scripts = Vec::new();
        if let Some(s) = scheme.scripts() {
            scripts = s.to_vec();
        }
        for val in vals {
            scripts.push(val.to_string())
        }
        scripts.retain(|x| fs::metadata(x).is_ok());
        scheme.set_scripts(Some(scripts));
    }

    if let Some(_) = app.values_of("pattern") {
        let vals: Vec<&str> = app.values_of("pattern").unwrap().collect();
        let mut patterns = Vec::new();
        for val in vals {
            let s: Vec<&str> = val.split_terminator(':').collect();
            // TODO: better error
            if s.len() == 2 { 
                if fs::metadata(s[0]).is_ok() && fs::metadata(s[1]).is_ok() {
                    patterns.push((s[0].to_string(), s[1].to_string()));
                };
            }
        }
        scheme.set_patterns(Some(patterns));
    }



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
