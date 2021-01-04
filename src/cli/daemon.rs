use anyhow::Result;
use std::path::PathBuf;
use std::{thread, time};
use daemonize::Daemonize;
use std::sync::mpsc::{channel, Sender};

use crate::var;
use crate::helper;
use crate::scheme::*;
use crate::gen::write;
use crate::fun::fifo;
use crate::cli::create;

pub fn run(app: &clap::ArgMatches, output: &mut WRITE, scheme: &mut SCHEME) -> Result<()> {
    let sub = app.subcommand_matches("daemon").unwrap();
    var::defs::concatinate(scheme);
    var::envi::concatinate(scheme);
    var::args::concatinate(app, scheme);
    var::pipe::concatinate(scheme);


    if atty::isnt(atty::Stream::Stdout) {
        println!("{}", "---");
    } else {
        if let Some(arg) = sub.value_of("action") {
            let mut lule_pipe = std::env::temp_dir(); lule_pipe.push("lule_pipe");
            if arg ==  "start" {
                deamoned(output, scheme)?;
            }
            if arg ==  "next" {
                helper::write_to_file(lule_pipe.clone(), "stop".as_bytes());
            }
            if arg ==  "stop" {
                helper::write_to_file(lule_pipe.clone(), "stop".as_bytes());
            }
            if arg ==  "detach" {
                let stdout = std::fs::File::create("/tmp/daemon.out").unwrap();
                let stderr = std::fs::File::create("/tmp/daemon.err").unwrap();
                let mut lule_pid = std::env::temp_dir();
                lule_pid.push("lule_pid");
                let lule = Daemonize::new()
                    .pid_file(lule_pid.to_str().unwrap()) // Every method except `new` and `start`
                    .chown_pid_file(true)      // is optional, see `Daemonize` documentation
                    .working_directory("/tmp") // for default behaviour.
                    .user(1000)
                    .group(1000)
                    .umask(0o777)
                    .stdout(stdout)
                    .stderr(stderr);
                match lule.start() {
                    Ok(_) => deamoned(output, scheme)?,
                    Err(e) => eprintln!("Error, {}", e),
                }
            }
        }
    }
    Ok(())

}

fn deamoned(output: &mut WRITE, scheme: &mut SCHEME) -> Result<()> {
    create::new_palette(output, scheme)?;

    let (timetx, timerx) = channel::<bool>();
    let timer = scheme.looop().unwrap().clone();
    thread::spawn(move || { time_to_sleep(timer, timetx ) });

    let mut lule_pipe = std::env::temp_dir(); lule_pipe.push("lule_pipe");
    let (pipetx, piperx) = channel::<String>();
    thread::spawn(move|| { read_file(lule_pipe, pipetx); });

    loop {
        let jsonified = serde_json::to_value(&scheme).unwrap();
        println!("{}", serde_json::to_string_pretty(&jsonified).unwrap());
        'inner: loop {
            if let Ok(content) = piperx.try_recv() {
                if let Ok(profile) = write::json_to_scheme(content.clone()) {
                    set_colors(output, scheme, &mut profile.clone())?;
                    break 'inner;
                } else if content.trim() == "next" {
                    create::new_palette(output, scheme)?;
                    break 'inner;
                } else if content.trim() == "stop" {
                    std::process::exit(0);
                } else {
                    println!("{} \n\n^^^ is not a valid json", content);
                }
            };
            if timerx.try_recv().is_ok() { 
                println!("time-looped");
                create::new_palette(output, scheme)?;
                break 'inner 
            }
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}

fn set_colors(output: &mut WRITE, scheme: &mut SCHEME, new_scheme: &mut SCHEME) -> Result<()> {
    new_scheme.set_walldir(None);
    new_scheme.set_image(None);
    scheme.modi(new_scheme);
    create::old_palette(output, scheme)?;
    Ok(())
}

fn read_file(pipe_name: PathBuf, sender: Sender<String>) {
    loop{
        let pipe = fifo::Pipe::new(pipe_name.clone());
        pipe.ensure_exists().unwrap();
        let reader = pipe.open_read();
        let content = reader.string().unwrap();
        sender.send(content).ok();
    }
}


fn time_to_sleep(time: usize, sender: Sender<bool>)  {
    loop{
        for _ in 0..time {
            thread::sleep(time::Duration::from_secs(1));
        }
        sender.send(true).ok();
    }
}
