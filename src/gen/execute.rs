use std::process::Command;
use crate::scheme::*;

fn external_command(script: &str){
    Command::new("bash")
                .arg("-c")
                .arg(script)
                .output()
                .expect("failed to execute process").stdout;
}

pub fn command_execution(scheme: &mut SCHEME) {
    if let Some(scripts) = scheme.scripts() {
        for s in scripts.iter() {
            if std::fs::metadata(&s).is_ok() {
                external_command(s);
                println!("running: {}", s)
            } else {
                //TODO: better error handle
                println!("{} is not a valid file", s )
            }
        }
    }
}

