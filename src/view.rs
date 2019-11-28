use std::io;
use std::io::Write;

use crate::Status;


pub fn display(status: &Status) {

    header(&status);
    
    song_list(&status);

    println!("\n \x1B[1m[a]\x1B[0m show All ({})", status.songs.len());
    println!(" \x1B[1m[n]\x1B[0m add New");
    println!(" \x1B[1m[v]\x1B[0m change Volume");
    println!(" \x1B[1m[s]\x1B[0m Stop");
    println!(" \x1B[1m[i]\x1B[0m Info");
    println!(" \x1B[1m[q]\x1B[0m Quit");


    print!("\n > ");
    let _ = io::stdout().flush();

}

fn song_list(status: &Status) {

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

pub fn header(status: &Status) {

    print!("\x1B[2J\x1B[H\n");
    println!("  ╔════════════════════╗ Song: {}", status.song[1]);
    println!("  ║ Cli \x1B[96mRepeat\x1B[0m in rust ║  vol: {}", status.volume);
    println!("  ╚════════════════════╝\n");

}