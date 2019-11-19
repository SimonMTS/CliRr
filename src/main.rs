use std::io::{self, Write};
use std::path::Path;

use std::fs;
use std::fs::File;
use std::fs::OpenOptions;

extern crate reqwest;
use serde::{Deserialize};

mod vlc_wrapper;
use crate::vlc_wrapper::{play, play_handeler_setup};

extern crate kernel32;

use crossterm::{ExecutableCommand, terminal};


fn main() {

    // just to set TERM flag ...
    io::stdout().execute(terminal::Clear(terminal::ClearType::All)).expect("Something went getting the handle");

    setup_checks();
    
    play_handeler_setup();

    menu(false);

}

fn menu(all: bool) {

    let contents = fs::read_to_string("./store.CliRr")
        .expect("Something went wrong reading the file");

    let mut songs: Vec<&str> = contents.lines().collect();

    display_menu(&songs, all);

    print!("\n Choice: ");
    let _ = io::stdout().flush();


    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() { s.pop(); }
    if let Some('\r')=s.chars().next_back() { s.pop(); }

    if s == "a" {

        menu(!all);

    } else if s.starts_with("n ") {
        let id = s.trim_start_matches("n ");
        let title = is_valid_id(&id);

        if title == "" {
            menu(all);
        }

        let song = format!("{}|||{}", id, title);

        songs.insert(0, &song);
        save_file(&songs);

        display_menu(&songs, false);

        play( song.split("|||").collect() );

        menu(false);

    }  else if s == "i" {

        display_info();

        menu(false);

    } else if s == "q" {

        return;

    } else {
        let mut song_int: i32 = 1;

        match s.parse::<i32>() {
            Ok(n) => song_int = n -1,
            Err(_e) => menu(all)
        }

        if song_int >= songs.len() as i32 {
            menu(all);
        } else {

            let song: Vec<&str> = songs[song_int as usize].split("|||").collect();
            
            update_file( &mut songs, song_int );
            display_menu(&songs, false);

            play( song );

            menu(false);

        }
    }

}


fn display_menu(songs: &Vec<&str>, all: bool) {

    print!("\x1B[2J\x1B[H\n");
    println!("  ╔════════════════════╗");
    println!("  ║ Cli \x1B[96mRepeat\x1B[0m in rust ║");
    println!("  ╚════════════════════╝\n");

    display_options(songs, all);

    println!("\n [a] show All ({})", songs.len());
    println!(" [n] add New");
    println!(" [i] Info");
    println!(" [q] Quit");

}

fn display_options(songs: &Vec<&str>, all: bool) {

    let songs_len: i32 = songs.len() as i32;

    for i in 0..songs_len {
        if i >= 5 && !all { break; }

        let song = songs[i as usize].to_string();

        let split_song: Vec<&str> = song.split("|||").collect();

        if i < 9 && songs_len > 9 && all {
            print!(" ");
        }

        println!(" [{}] {}", i+1, split_song[1]);
    }    

}

fn display_info() {

    print!("\x1B[2J\x1B[H\n");
    println!("  ╔════════════════════╗");
    println!("  ║ Cli \x1B[96mRepeat\x1B[0m in rust ║");
    println!("  ╚════════════════════╝\n");

    println!(" [a] show All:\n   Shows the full list of stored songs.\n");
    println!(" [n] add New: \n   \x1B[1m\"n <video_id>\"\x1B[0m, adds the youtube video_id to the top of the list, and starts playing it.\n");

    println!(" <Press enter to continue>");

    let mut _s = String::new();
    io::stdin().read_line(&mut _s).expect("Did not enter a correct string");

}


fn update_file(songs: &mut Vec<&str>, song_int: i32) {

    let selected_song = songs.remove(song_int as usize);
    songs.insert(0, selected_song);

    save_file(songs);

}

fn save_file(songs: &Vec<&str>) {

    let _file = File::create("store.CliRr");

    let mut file = OpenOptions::new()
        .append(true)
        .open("store.CliRr")
        .unwrap();

    let songs_len: i32 = songs.len() as i32;
    for i in 0..songs_len {

        if let Err(e) = writeln!(file, "{}", songs[i as usize].to_string()) {
            eprintln!("Couldn't write to file: {}", e);
        }

    }

}


fn setup_checks() {

    if !Path::new("store.CliRr").exists() {
        let _file = File::create("store.CliRr");
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