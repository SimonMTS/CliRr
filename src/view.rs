use std::io;
use std::io::Write;
use std::io::StdoutLock;

use crate::Status;


pub fn display(status: &Status) {

    let out = io::stdout();
    let mut lock = out.lock();

    header(&status, &mut lock);
    
    song_list(&status, &mut lock);

    writeln!(lock, "").expect("stdout err");
    writeln!(lock, " \x1B[1m[\x1B[0ma\x1B[1m]\x1B[0m show All ({})", status.songs.len()).expect("stdout err");
    writeln!(lock, " \x1B[1m[\x1B[0mn\x1B[1m]\x1B[0m add New").expect("stdout err");
    writeln!(lock, " \x1B[1m[\x1B[0mv\x1B[1m]\x1B[0m change Volume").expect("stdout err");
    writeln!(lock, " \x1B[1m[\x1B[0ms\x1B[1m]\x1B[0m Stop").expect("stdout err");
    writeln!(lock, " \x1B[1m[\x1B[0mi\x1B[1m]\x1B[0m Info").expect("stdout err");
    writeln!(lock, " \x1B[1m[\x1B[0mq\x1B[1m]\x1B[0m Quit").expect("stdout err");


    write!(lock, "\n > ").expect("stdout err");
    let _ = io::stdout().flush();

}

fn song_list(status: &Status, lock: &mut StdoutLock) {

    let songs_len: i32 = status.songs.len() as i32;

    for i in 0..songs_len {
        if i >= 5 && !status.show_all { break; }

        let song = &status.songs[i as usize];

        let split_song: Vec<&str> = song.split("|||").collect();

        if i < 9 && songs_len > 9 && status.show_all {
            write!(lock, " ").expect("stdout err");
        }

        writeln!(lock, " \x1B[36m[\x1B[0m{}\x1B[36m]\x1B[0m {}", i+1, split_song[1]).expect("stdout err");
    }    

}

fn header(status: &Status, lock: &mut StdoutLock) {

    write!(lock, "\x1B[2J\x1B[H\n").expect("stdout err");
    writeln!(lock, "  ╔════════════════════╗ Song: {}", status.song[1]).expect("stdout err");
    writeln!(lock, "  ║ Cli \x1B[96mRepeat\x1B[0m in rust ║  vol: {}", status.volume).expect("stdout err");
    writeln!(lock, "  ╚════════════════════╝\n").expect("stdout err");

}

pub fn info(status: &Status) {

    let out = io::stdout();
    let mut lock = out.lock();

    header(&status, &mut lock);
    
    writeln!(lock, " [a] show All:\n   Shows the full list of stored songs.\n").expect("stdout err");
    writeln!(lock, " [n] add New: \n   \x1B[1m\"n <video_id>\"\x1B[0m, adds the youtube video_id to the top of the list, and starts playing it.\n").expect("stdout err");

    writeln!(lock, " <Press enter to continue>").expect("stdout err");
}
