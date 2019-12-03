use crate::data_store;
use crate::vlc_wrapper as vlc;
use crate::Status;


pub fn exec(mut status: Status, input: String) -> Status{

    let song_int: i32;

    match input.parse::<i32>() {
        Ok(n) => song_int = n -1,
        Err(_e) => song_int = 999
    }

    if song_int < status.songs.len() as i32 {

        status.song = status.songs[song_int as usize].split("|||").map(|s| s.to_string()).collect();
    
        vlc::play( status.song.clone(), status.volume );

        let selected_song = status.songs.remove(song_int as usize);
        status.songs.insert(0, selected_song);

        data_store::save( &status );

    }

    return status;
}