use std::io::Write;
use std::fs::File;
use std::fs::OpenOptions;

use crate::Status;


pub fn save(status: &Status) {

    let _file = File::create("store.CliRr");

    let mut file = OpenOptions::new()
        .append(true)
        .open("store.CliRr")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", status.volume.to_string()) {
        eprintln!("Couldn't write to file: {}", e);
    }

    let songs_len: i32 = status.songs.len() as i32;
    for i in 0..songs_len {

        if let Err(e) = writeln!(file, "{}", status.songs[i as usize].to_string()) {
            eprintln!("Couldn't write to file: {}", e);
        }

    }

}