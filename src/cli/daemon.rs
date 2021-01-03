use anyhow::Result;
use std::path::PathBuf;
use std::{thread, time};
use daemonize::Daemonize;
use std::sync::mpsc::{channel, Sender};

use crate::var;
use crate::helper;
use crate::scheme::*;
use crate::gen::palette;
use crate::gen::generate;
use crate::gen::execute;
use crate::gen::write;
use crate::fun::fifo;

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
            if arg ==  "start" {
                deamoned(output, scheme)?;
            }
            if arg ==  "stop" {
                let mut sigterm = std::env::temp_dir();
                sigterm.push("lule_sigterm");
                helper::write_to_file(sigterm, "stop".as_bytes());
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
    let (sigtermtx, sigtermrx) = channel::<String>();
    let mut sigterm = std::env::temp_dir(); sigterm.push("lule_sigterm");
    thread::spawn(move|| { read_file(sigterm, sigtermtx); });

    let (timetx, timerx) = channel::<bool>();
    let timer = scheme.looop().unwrap().clone();
    thread::spawn(move || { time_to_sleep(timer, timetx ) });

    let mut lule_pipe = std::env::temp_dir(); lule_pipe.push("lule_pipe");
    let (pipetx, piperx) = channel::<String>();
    thread::spawn(move|| { read_file(lule_pipe, pipetx); });

    loop {
        'inner: loop {
            if let Ok(content) = piperx.try_recv() {
                if let Ok(profile) = write::json_to_scheme(content.clone()) {
                    set_colors(output, scheme, &mut profile.clone())?;
                } else {
                    println!("{} \n\n^^^ is not a valid json", content);
                }
                break 'inner;
            };
            if timerx.try_recv().is_ok() { 
                println!("time-looped");
                break 'inner 
            } else if sigtermrx.try_recv().is_ok() { 
                std::process::exit(0);
            }
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}

fn set_colors(output: &mut WRITE, scheme: &mut SCHEME, new_scheme: &mut SCHEME) -> Result<()> {
    let jsonified_old = serde_json::to_value(&scheme).unwrap();
    println!("{}", serde_json::to_string_pretty(&jsonified_old).unwrap());

    new_scheme.set_walldir(scheme.walldir().clone());
    new_scheme.set_image(scheme.image().clone());
    scheme.modi(new_scheme);

    let palette: Vec<String>;
    if let Some(content) = scheme.palette() {
        match content.as_str() {
            "pigment" => {
                palette = palette::palette_from_image(scheme.image().clone().unwrap());
                helper::write_temp_file("lule_palette", palette.join("\n").as_bytes());
                scheme.set_colors(Some(palette));
            },
            _ => unreachable!(),
        };
    }

    output.set_theme(scheme.theme().clone().unwrap());
    output.set_colors(generate::get_all_colors(scheme));
    output.set_wallpaper(scheme.image().clone().unwrap());
    
    let values = write::output_to_json(output, false);
    write::write_temp(&output);
    write::write_cache(&scheme);
    write::write_cache_json(scheme, values);
    execute::external_command();

    let jsonified_new = serde_json::to_value(&scheme).unwrap();
    println!("{}", serde_json::to_string_pretty(&jsonified_new).unwrap());
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
