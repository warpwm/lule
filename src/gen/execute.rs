use std::process::Command;
use crate::scheme::*;

pub fn external_command(scheme: &mut SCHEME){
    if let Some(scripts) = scheme.scripts() {
        for script in scripts.iter() {
            println!("{}", script)
        }
    }
    Command::new("bash")
                .arg("-c")
                .arg("/home/bresilla/code/proj/warp/lule/scripts/lule_colors")
                .output()
                .expect("failed to execute process").stdout;
}

