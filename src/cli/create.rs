use crate::generator::apply;
use crate::scheme::*;
use crate::var;
use anyhow::Result;

pub fn run(app: &clap::ArgMatches, scheme: &mut Scheme) -> Result<()> {
    let sub = app.subcommand_matches("create").unwrap();
    var::concatinate(app, scheme);

    if atty::isnt(atty::Stream::Stdout) {
        // write::write_temp(&scheme);
        // println!("{}", &values);
    } else if let Some(arg) = sub.value_of("action") {
        if arg == "set" {
            apply::write_colors(scheme, false)?;
        }
        if arg == "regen" {
            apply::write_colors(scheme, true)?;
        }
    }
    Ok(())
}
