use std::process::Command;
use std::thread;
use std::process;

extern crate ctrlc;

pub fn play( song: Vec<String>, volume: f32 ) {

    stop();

    let id = song[0].to_string();
    let title = song[1].to_string();

    println!("\n Playing: {}", title);

    thread::spawn(move || {
        Command::new("vlc")
            .arg("-I dummy")
            .arg("--dummy-quiet")
            .arg("--vout=\"none\"")
            .arg("--one-instance")
            .arg("--repeat")
            .arg( format!("--mmdevice-volume={}", volume/100.0, ) )
            .arg( format!("https://www.youtube.com/watch?v={}", id))
            .output()
            .expect("failed to download/play video");
    });

}

pub fn stop() {
    
    Command::new("vlc")
        .arg("vlc://quit")
        .arg("-I dummy")
        .arg("--dummy-quiet")
        .arg("--vout=\"none\"")
        .arg("--one-instance")
        .output()
        .expect("failed to stop vlc");

}

pub fn play_handeler_setup() {

    ctrlc::set_handler(move || {
        
        Command::new("vlc")
            .arg("vlc://quit")
            .arg("-I dummy")
            .arg("--dummy-quiet")
            .arg("--vout=\"none\"")
            .arg("--one-instance")
            .output()
            .expect("failed to stop vlc");

        process::exit(0);

    }).expect("Error setting Ctrl-C handler");

}