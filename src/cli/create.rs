use anyhow::Result;
use crate::var;
use crate::scheme::*;
use crate::gen::apply;

pub fn run(app: &clap::ArgMatches, scheme: &mut SCHEME) -> Result<()> {
    let sub = app.subcommand_matches("create").unwrap();
    var::concatinate(app, scheme);

    if atty::isnt(atty::Stream::Stdout) {
        // write::write_temp(&scheme);
        // println!("{}", &values);
    } else {
        if let Some(arg) = sub.value_of("action") {
            if arg ==  "set" {
                apply::write_colors(scheme, false)?;
            }
            if arg ==  "regen" {
                apply::write_colors(scheme, true)?;
            }
        }
    }
    Ok(())
}
