use std::process::{Command, Stdio};
use std::process;

use rodio::Source;
use rodio::Sink;
use std::fs::File;
use std::io::BufReader;
use std::env;
use std::path::Path;

extern crate ctrlc;

static mut CHILD: Option<process::Child> = None;
static mut SINK: Option<rodio::Sink> = None;


pub fn play( song: Vec<String>, volume: f32 ) {

    stop();

    let id = song[0].to_string();

    unsafe {
        if cfg!(windows) {

            let filename = get_filename().replace("\\", "/");

            if !Path::new( &format!("{}data/{}.mp3", &filename, id) ).exists() {
                let _vid = Command::new("youtube-dl")
                    .arg( format!("--output={}data/{}.%(ext)s", &filename, id) )
                    .arg("--audio-format=mp3")
                    .arg("--extract-audio")
                    .arg("--no-warnings")
                    .arg("-q")
                    .arg( format!("{}", id) )
                    .output()
                    .expect("failed to download video");
            }
            
            let device = rodio::default_output_device().unwrap();
            let sink = Sink::new(&device);

            let file = File::open( format!("{}data/{}.mp3", &filename, id) ).unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            let rep_source = source.repeat_infinite();
            
            sink.append(rep_source);
            sink.set_volume(volume/1000.0);
            SINK = Some(sink);

        } else if cfg!(unix) {
            
            CHILD = Some(Command::new("vlc")
                .arg("-Vvdummy")
                .arg("-I dummy")
                .arg("--repeat")
                // .arg( format!("--waveout-volume={}", volume/100.0, ) )
                .arg( format!("https://www.youtube.com/watch?v={}", id) )
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("failed to download/play video"));

        }
    }

}

pub fn stop() {
    
    unsafe {
        match CHILD {
            Some(ref mut x) => {
                x.kill().expect("command wasn't running");
                CHILD = None;
            },
            None => print!("")
        }

        match SINK {
                Some(ref mut x) => {
                    x.stop();
                },
                None => print!("")
            }
    }

}

pub fn play_handeler_setup() {

    ctrlc::set_handler(move || {
        
        stop();
        process::exit(0);

    }).expect("Error setting Ctrl-C handler");

}

pub fn path_is_set() -> bool {
    
    return Command::new("vlc")
        .arg("vlc://quit")
        .arg("-Vvdummy")
        .arg("-I dummy")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .is_ok();

}


fn get_filename() -> String {

    match env::current_exe() {
        Ok(exe_path) => {
            return exe_path.display().to_string()
                .trim_end_matches(".exe")
                .trim_end_matches("clirr")
                .trim_end_matches("CliRr").to_string();
        }
        Err(e) => {
            println!("failed to get current exe path: {}", e);
            return "".to_string();
        }
    };

}