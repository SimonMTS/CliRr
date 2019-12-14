use std::process::{Command, Stdio};
use std::process;

extern crate ctrlc;

static mut CHILD: Option<process::Child> = None;


pub fn play( song: Vec<String>, _volume: f32 ) {

    stop();

    let id = song[0].to_string();

    unsafe {
        CHILD = Some(Command::new("vlc")
            .arg("-Vvdummy")
            .arg("-I dummy")
            .arg("--repeat")
            // .arg( format!("--waveout-volume={}", volume/100.0, ) )
            .arg( format!("https://www.youtube.com/watch?v={}", id))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("failed to download/play video"));
    }

}

pub fn stop() {
    
    unsafe {
        match CHILD {
            Some(ref mut x) => {
                x.kill().expect("command wasn't running");
                CHILD = None;
            },
            None    => print!("")
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