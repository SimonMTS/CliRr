use serde::{Deserialize};

use crate::data_store;
use crate::vlc_wrapper as vlc;
use crate::Status;


pub fn exec(mut status: Status, input: String) -> Status{

    let id = input.trim_start_matches("n ");
    let title = is_valid_id(&id);

    if title != "" {

        let song_str = format!("{}|||{}", id, title);

        status.song = song_str.split("|||").map(|s| s.to_string()).collect();
        status.songs.insert(0, song_str);
        status.show_all = false;

        data_store::save( &status );
        
        vlc::play( status.song.clone(), status.volume );

    }

    return status;
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