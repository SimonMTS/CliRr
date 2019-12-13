use std::io::Write;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::path::Path;
use std::env;

use crate::Status;


pub fn save(status: &Status) {

    let filename = get_filename();
    let _file = File::create(&filename);

    let mut file = OpenOptions::new()
        .append(true)
        .open(&filename)
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

    let filename = get_filename();
    let vol_int: f32;
    let contents = fs::read_to_string(&filename)
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

    let filename = get_filename();

    if !Path::new(&filename).exists() {
        let _file = File::create(&filename);
    }
    

    let contents = fs::read_to_string(&filename).expect("Something went wrong reading the file");
    let line = contents.lines().next();
    
    if line == None {

        let mut file = OpenOptions::new()
            .append(true)
            .open(&filename)
            .unwrap();

        if let Err(e) = writeln!(file, "{}", "100") {
            eprintln!("Couldn't write to file: {}", e);
        }

    }

}


fn get_filename() -> String {

    match env::current_exe() {
        Ok(exe_path) => {
            return exe_path.display().to_string()
                .trim_end_matches(".exe")
                .trim_end_matches("clirr")
                .trim_end_matches("CliRr").to_string() + 
                ".CliRr";
        }
        Err(e) => {
            println!("failed to get current exe path: {}", e);
            return "".to_string();
        }
    };

}