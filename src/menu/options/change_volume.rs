use crate::data_store;
use crate::vlc_wrapper as vlc;
use crate::Status;


pub fn exec(mut status: Status, input: String) -> Status {

    let vol = input.trim_start_matches("v").trim();
    let vol_int: f32;

    match vol.parse::<f32>() {
        Ok(n) => vol_int = n,
        Err(_e) => vol_int = -1.0
    }

    if vol_int > 0.0 {
        if vol_int > 2000.0 {
            status.volume = 2000.0;
        } else {
            status.volume = vol_int;
        }

        vlc::play( status.song.clone(), status.volume );

        data_store::save( &status );
    }

    return status;
}