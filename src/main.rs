use std::process::Command;
use std::thread;
use std::process;

extern crate ctrlc;

use std::env;
use std::io;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Playing...");

    thread::spawn(move || {
        Command::new("vlc")
            .arg("-I dummy")
            .arg("--dummy-quiet")
            .arg("--vout=\"none\"")
            .arg("--one-instance")
            .arg("--repeat")
            .arg( format!("https://www.youtube.com/watch?v={}", args[1]))
            .output()
            .expect("failed to download video");
    });


    ctrlc::set_handler(move || {
        
        Command::new("vlc")
            .arg("vlc://quit")
            .arg("--one-instance")
            .output()
            .expect("failed to download video");

        println!("Done");
        process::exit(0);

    }).expect("Error setting Ctrl-C handler");


    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();

    Command::new("vlc")
        .arg("vlc://quit")
        .arg("--one-instance")
        .output()
        .expect("failed to download video");

    println!("Done");
}
