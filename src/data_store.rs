use std::io::Write;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::path::Path;

use crate::Status;


pub fn save(status: &Status) {

    let _file = File::create(".CliRr");

    let mut file = OpenOptions::new()
        .append(true)
        .open(".CliRr")
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

pub fn read() -> Status {

    let vol_int: f32;
    let contents = fs::read_to_string(".CliRr")
        .expect("Something went wrong reading the file");
    let mut lines = contents.lines();

    let vol = lines.next().unwrap().to_string();

    match vol.parse::<f32>() {
        Ok(n) => vol_int = n,
        Err(_e) => vol_int = 100.0
    }

    let status = Status {
        volume: vol_int,
        song: "|||  ".split("|||").map(|s| s.to_string()).collect(),
        playing: false,

        show_all: -1,
        songs: lines.map(|s| s.to_string()).collect()
    };

    return status;
}


pub fn make_data_store_valid() {

    if !Path::new(".CliRr").exists() {
        let _file = File::create(".CliRr");
    }
    

    let contents = fs::read_to_string(".CliRr").expect("Something went wrong reading the file");
    let line = contents.lines().next();
    
    if line == None {

        let mut file = OpenOptions::new()
            .append(true)
            .open(".CliRr")
            .unwrap();

        if let Err(e) = writeln!(file, "{}", "100") {
            eprintln!("Couldn't write to file: {}", e);
        }

    }

}