use std::io::{self, Write};
use std::path::Path;

use std::fs;
use std::fs::File;
use std::fs::OpenOptions;

extern crate reqwest;
use serde::{Deserialize};

extern crate kernel32;

use crossterm::{ExecutableCommand, terminal};

mod vlc_wrapper;
use crate::vlc_wrapper::{play, stop, play_handeler_setup};


#[derive(Clone)]
struct Status {
    volume: f32,
    song: Vec<String>,
    playing: bool,

    show_all: bool,
    songs: Vec<String>
}

fn main() {

    // just to set TERM flag ...
    io::stdout().execute(terminal::Clear(terminal::ClearType::All)).expect("Something went getting the handle");

    setup_checks();
    play_handeler_setup();


    let contents = fs::read_to_string("./store.CliRr")
        .expect("Something went wrong reading the file");
    let mut lines = contents.lines();

    let vol = lines.next().unwrap().to_string();

    let vol_int: f32;
    match vol.parse::<f32>() {
        Ok(n) => vol_int = n,
        Err(_e) => vol_int = 100.0
    }

    let status = Status {
        volume: vol_int,
        song: "|||".split("|||").map(|s| s.to_string()).collect(),
        playing: false,

        show_all: false,
        songs: lines.map(|s| s.to_string()).collect()
    };


    menu(status);

}

fn menu(mut status: Status) {
    
    display_menu(&status);

    print!("\n > ");
    let _ = io::stdout().flush();

    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() { s.pop(); }
    if let Some('\r')=s.chars().next_back() { s.pop(); }

    if s == "a" {

        status.show_all = !status.show_all;

    } else if s.starts_with("v ") {

        let vol = s.trim_start_matches("v ");
        let vol_int: f32;

        match vol.parse::<f32>() {
            Ok(n) => vol_int = n,
            Err(_e) => vol_int = 999.9
        }

        if vol_int <= 200.0 {
            status.volume = vol_int;

            play( status.song.clone(), status.volume );

            save_file( &status );
        }

    } else if s.starts_with("n ") {

        let id = s.trim_start_matches("n ");
        let title = is_valid_id(&id);

        if title != "" {

            let song_str = format!("{}|||{}", id, title);

            status.song = song_str.split("|||").map(|s| s.to_string()).collect();
            status.songs.insert(0, song_str);
            status.show_all = false;

            save_file( &status );
            
            play( status.song.clone(), status.volume );

        }

    }  else if s == "i" {

        display_info(&status);

        status.show_all = false;

    } else if s == "q" {

        stop();

        return;

    } else if s == "s" {

        status.song = "|||".split("|||").map(|s| s.to_string()).collect();

        stop();

    } else {

        let song_int: i32;

        match s.parse::<i32>() {
            Ok(n) => song_int = n -1,
            Err(_e) => song_int = 999
        }

        if song_int < status.songs.len() as i32 {

            status.song = status.songs[song_int as usize].split("|||").map(|s| s.to_string()).collect();
            status.show_all = false;
            
            play( status.song.clone(), status.volume );

            let selected_song = status.songs.remove(song_int as usize);
            status.songs.insert(0, selected_song);

            save_file( &status );

        }

    }

    menu(status);
}


fn display_menu(status: &Status) {

    print!("\x1B[2J\x1B[H\n");
    println!("  ╔════════════════════╗ Song: {}", status.song[1]);
    println!("  ║ Cli \x1B[96mRepeat\x1B[0m in rust ║  vol: {}", status.volume);
    println!("  ╚════════════════════╝\n");
    
    display_options(&status);

    println!("\n \x1B[1m[a]\x1B[0m show All ({})", status.songs.len());
    println!(" \x1B[1m[n]\x1B[0m add New");
    println!(" \x1B[1m[v]\x1B[0m change Volume");
    println!(" \x1B[1m[s]\x1B[0m Stop");
    println!(" \x1B[1m[i]\x1B[0m Info");
    println!(" \x1B[1m[q]\x1B[0m Quit");

}

fn display_options(status: &Status) {

    let songs_len: i32 = status.songs.len() as i32;

    for i in 0..songs_len {
        if i >= 5 && !status.show_all { break; }

        let song = status.songs[i as usize].to_string();

        let split_song: Vec<&str> = song.split("|||").collect();

        if i < 9 && songs_len > 9 && status.show_all {
            print!(" ");
        }

        println!(" \x1B[1m[{}]\x1B[0m {}", i+1, split_song[1]);
    }    

}

fn display_info(status: &Status) {

    print!("\x1B[2J\x1B[H\n");
    println!("  ╔════════════════════╗ Song: {}", status.song[1]);
    println!("  ║ Cli \x1B[96mRepeat\x1B[0m in rust ║  vol: {}", status.volume);
    println!("  ╚════════════════════╝\n");

    println!(" [a] show All:\n   Shows the full list of stored songs.\n");
    println!(" [n] add New: \n   \x1B[1m\"n <video_id>\"\x1B[0m, adds the youtube video_id to the top of the list, and starts playing it.\n");

    println!(" <Press enter to continue>");

    let mut _s = String::new();
    io::stdin().read_line(&mut _s).expect("Did not enter a correct string");

}


fn save_file(status: &Status) {

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


fn setup_checks() {

    if !Path::new("store.CliRr").exists() {
        let _file = File::create("store.CliRr");

        let mut file = OpenOptions::new()
            .append(true)
            .open("store.CliRr")
            .unwrap();

        if let Err(e) = writeln!(file, "{}", "100") {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    // vlc path checks

}


fn is_valid_id(id: &str) -> String {

    let mut response = reqwest::get(&format!("https://www.youtube.com/oembed?format=json&url=http://www.youtube.com/watch?v={}", id))
        .expect("API call failed");

    #[derive(Deserialize)]
    struct Video {
        title: String,
    }
    let json = response.json::<Video>().expect("JSON decode failed");

    if response.status() != 404 {
        return json.title;
    } else {
        return "".to_string();
    }

}