use std::io;
use std::io::Write;
use std::io::StdoutLock;

use crate::Status;


pub fn display(status: &Status) {

    let out = io::stdout();
    let mut lock = out.lock();

    header(&status, &mut lock);
    
    song_list(&status, &mut lock);

    if status.show_all == -1 {
        writeln!(lock, "").expect("stdout err");
        writeln!(lock, "  \x1B[1m[\x1B[0ma\x1B[1m]\x1B[0m show All ({})", status.songs.len()).expect("stdout err");
        writeln!(lock, "  \x1B[1m[\x1B[0mn\x1B[1m]\x1B[0m add New").expect("stdout err");
        writeln!(lock, "  \x1B[1m[\x1B[0mv\x1B[1m]\x1B[0m change Volume").expect("stdout err");
        writeln!(lock, "  \x1B[1m[\x1B[0ms\x1B[1m]\x1B[0m Stop").expect("stdout err");
        writeln!(lock, "  \x1B[1m[\x1B[0mi\x1B[1m]\x1B[0m Info").expect("stdout err");
        writeln!(lock, "  \x1B[1m[\x1B[0mq\x1B[1m]\x1B[0m Quit").expect("stdout err");
    }


    write!(lock, "\n  \x1B[96m>\x1B[36m>\x1B[37m> ").expect("stdout err");
    let _ = io::stdout().flush();

}

fn song_list(status: &Status, lock: &mut StdoutLock) {

    let songs_len: i32 = status.songs.len() as i32;
    let start_value;
    let end_value;

    if status.show_all == -1 {
        start_value = 0;
        end_value = 5;
    } else if status.show_all == -2 {
        start_value = 0;
        end_value = songs_len;
    } else {
        start_value = status.show_all;
        end_value = status.show_all + 15;
    }

    for i in start_value..end_value {
        if i >= songs_len { break; }

        let song = &status.songs[i as usize];
        let split_song: Vec<&str> = song.split("|||").collect();

        if start_value < 9 && i < 9 && end_value > 9 {
            write!(lock, " ").expect("stdout err");
        }

        if split_song[1].contains("-") {
            writeln!(lock, "  \x1B[36m[\x1B[37m{}\x1B[36m]\x1B[0m {}\x1B[0m", i+1, split_song[1]
                .replacen("-", "-\x1B[96m", 1)
                .replacen("(", "\x1B[0m(", 1))
                .replacen("[", "\x1B[0m[", 1))
                .expect("stdout err");
        } else {
            writeln!(lock, "  \x1B[36m[\x1B[37m{}\x1B[36m]\x1B[0m \x1B[96m{}\x1B[0m", i+1, split_song[1]).expect("stdout err");
        }
    }    

}

fn header(status: &Status, lock: &mut StdoutLock) {

    // write all spaces upto cursor position, then move cursor to 1:1 of viewport
    write!(lock, "\x1B[2J\x1B[H\n").expect("stdout err");

    writeln!(lock, "  ╔════════════════════╗ Song: \x1B[30m\x1B[46m {} \x1B[0m", status.song[1]).expect("stdout err");
    writeln!(lock, "  ║ Cli \x1B[96mRepeat\x1B[0m in rust ║  vol: \x1B[30m\x1B[46m {} \x1B[0m", status.volume).expect("stdout err");
    writeln!(lock, "  ╚════════════════════╝\n").expect("stdout err");

}

pub fn info(status: &Status) {

    let out = io::stdout();
    let mut lock = out.lock();

    header(&status, &mut lock);
    
    writeln!(lock, "  [a] show All:\n    Allows viewing all songs. (space between 'a' & number is optional)").expect("stdout err");
    writeln!(lock, "    \x1B[30m\x1B[46m\"a -2\"\x1B[0m, really shows all songs.").expect("stdout err");
    writeln!(lock, "    \x1B[30m\x1B[46m\"a -1\"\x1B[0m, show only 5 songs.").expect("stdout err");
    writeln!(lock, "    \x1B[30m\x1B[46m\"a <0..*>\"\x1B[0m, shows 15 songs starting from n.").expect("stdout err");
    writeln!(lock, "    \x1B[30m\x1B[46m\"a\"\x1B[0m, toggle between -1 & 0.\n").expect("stdout err");
    
    writeln!(lock, "  [n] add New: \n    \x1B[30m\x1B[46m\"n <video_id>\"\x1B[0m, adds the youtube video_id to the top of the list, and starts playing it.\n").expect("stdout err");
    writeln!(lock, "  [v] change Volume: \n    \x1B[30m\x1B[46m\"v <0-200>\"\x1B[0m, restarts the current song with the selected volume. (space between 'v' & number is optional)\n").expect("stdout err");

    writeln!(lock, " <Press enter to continue>").expect("stdout err");
}
