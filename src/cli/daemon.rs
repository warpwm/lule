use crate::var;
use crate::scheme::*;
// use daemonize::Daemonize;
use std::{thread, time};
use anyhow::Result;
use nix::unistd;
use nix::sys::stat;
use std::path::{Path, PathBuf};

use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::Duration;
use crate::helper;
use crate::gen::write;
use crate::fun::fifo;

pub fn run(app: &clap::ArgMatches, output: &mut WRITE, scheme: &mut SCHEME) -> Result<()> {

    // deamoned(app, output, scheme);
    var::defs::concatinate(scheme);
    var::envi::concatinate(scheme);
    var::args::concatinate(app, scheme);
    var::pipe::concatinate(scheme);

    // let stdout = std::fs::File::create("/tmp/daemon.out").unwrap();
    // let stderr = std::fs::File::create("/tmp/daemon.err").unwrap();

    // let mut lule_pid = std::env::temp_dir();
    // lule_pid.push("lule_pid");

    // let lule = Daemonize::new()
    //     .pid_file(lule_pid.to_str().unwrap()) // Every method except `new` and `start`
    //     .chown_pid_file(true)      // is optional, see `Daemonize` documentation
    //     .working_directory("/tmp") // for default behaviour.
    //     .user(1000)
    //     .group(1000)
    //     .umask(0o777)
    //     .stdout(stdout)
    //     .stderr(stderr);

    // match lule.start() {
    //     // Ok(_) => rtm.block_on(deamoned(app, output, scheme)),
    //     Ok(_) => deamoned(app, output, scheme),
    //     Err(e) => eprintln!("Error, {}", e),
    // }

    deamoned(output, scheme)?;
    Ok(())
}

fn deamoned(_output: &mut WRITE, scheme: &mut SCHEME) -> Result<()> {
    let (timetx, timerx) = channel::<bool>();
    let timer = scheme.looop().unwrap().clone();
    thread::spawn(move || { time_to_sleep(timer, timetx ) });

    'outer: loop {
        let mut pipe_name = std::env::temp_dir();
        pipe_name.push("lule_pipe");
        let (pipetx, piperx) = channel::<String>();
        thread::spawn(move|| { read_file(pipe_name, pipetx); });
        'inner: loop {
            if let Ok(content) = piperx.try_recv() {
                if let Ok(profile) = write::json_to_scheme(content.clone()) {
                    let jsonified = serde_json::to_value(&profile).unwrap();
                    println!("{}", jsonified);
                } else {
                    println!("something bad happened");
                }
                break;
            };
            thread::sleep(time::Duration::from_millis(10));
            if timerx.try_recv().is_ok() { break 'outer }
        }
    }
    Ok(())
}

fn read_file(pipe_name: PathBuf, sender: Sender<String>) {
    let pipe = fifo::Pipe::new(pipe_name);
    pipe.ensure_exists().unwrap();
    let reader = pipe.open_read();
    let content = reader.string().unwrap();
    sender.send(content);
}


fn time_to_sleep(time: usize, sender: Sender<bool>){
    for i in 0..10 {
        thread::sleep(time::Duration::from_secs(1));
    }
    sender.send(true);
}
