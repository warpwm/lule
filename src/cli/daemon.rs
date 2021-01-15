use anyhow::Result;
use std::path::PathBuf;
use std::{thread, time};
use daemonize::Daemonize;
use std::sync::mpsc::{channel, Sender};

use crate::var;
use crate::fun::text;
use crate::scheme::*;
use crate::gen::write;
use crate::fun::fifo;
use crate::gen::apply;

pub fn run(app: &clap::ArgMatches, scheme: &mut SCHEME) -> Result<()> {
    let sub = app.subcommand_matches("daemon").unwrap();
    var::concatinate(app, scheme);


    if atty::isnt(atty::Stream::Stdout) {
        println!("{}", "you cant pipe out form this deamon");
    } else {
        if let Some(arg) = sub.value_of("action") {
            let mut lule_pipe = std::env::temp_dir(); lule_pipe.push("lule_pipe");
            if arg ==  "start" {
                deamoned(scheme)?;
            }
            if arg ==  "next" {
                text::write_to_file(lule_pipe.clone(), "stop".as_bytes());
            }
            if arg ==  "stop" {
                text::write_to_file(lule_pipe.clone(), "stop".as_bytes());
            }
            if arg ==  "detach" {
                let stdout = std::fs::File::create("/tmp/daemon.out").unwrap();
                let stderr = std::fs::File::create("/tmp/daemon.err").unwrap();
                let mut lule_pid = std::env::temp_dir(); lule_pid.push("lule.pid");
                let lule = Daemonize::new()
                    .pid_file(lule_pid)
                    .chown_pid_file(true)
                    .working_directory("/tmp")
                    .user(1000)
                    .group(1000)
                    .stdout(stdout)
                    .stderr(stderr);
                match lule.start() {
                    Ok(_) => deamoned(scheme)?,
                    Err(e) => eprintln!("Error, {}", e),
                }
            }
        }
    }
    Ok(())

}

fn deamoned(scheme: &mut SCHEME) -> Result<()> {
    let mut lule_pipe = std::env::temp_dir(); lule_pipe.push("lule_pipe");
    std::fs::remove_file(lule_pipe.clone()).ok();
    let (pipetx, piperx) = channel::<String>();
    thread::spawn(move|| { read_pipe(lule_pipe, pipetx); });

    let (timetx, timerx) = channel::<bool>();
    let timer = scheme.looop().unwrap().clone();
    thread::spawn(move || { time_to_sleep(timer, timetx ) });

    apply::write_colors(scheme, false)?;
    loop {
        let jsonified = serde_json::to_value(&scheme).unwrap();
        println!("{}", serde_json::to_string_pretty(&jsonified).unwrap());
        'inner: loop {
            if let Ok(content) = piperx.try_recv() {
                if let Ok(profile) = write::json_to_scheme(content.clone()) {
                    scheme.modi(&mut profile.clone());
                    println!("{}", scheme.theme().clone().unwrap());
                    apply::write_colors(scheme, false)?;
                    break 'inner;
                } else if content.trim() == "next" {
                    scheme.set_image(None);
                    apply::write_colors(scheme, false)?;
                    break 'inner;
                } else if content.trim() == "stop" {
                    std::process::exit(0);
                } else {
                    println!("{} \n\n^^^ is not a valid json", content);
                }
            };
            if timerx.try_recv().is_ok() { 
                scheme.set_image(None);
                apply::write_colors(scheme, false)?;
                break 'inner 
            }
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}

fn read_pipe(pipe_name: PathBuf, sender: Sender<String>) {
    loop{
        std::fs::remove_file(pipe_name.clone()).ok();
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
