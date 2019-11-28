use crate::data_store;
use crate::vlc_wrapper as vlc;
use crate::Status;


pub fn exec(mut status: Status, input: String) -> Status {

    let vol = input.trim_start_matches("v ");
    let vol_int: f32;

    match vol.parse::<f32>() {
        Ok(n) => vol_int = n,
        Err(_e) => vol_int = 999.9
    }

    if vol_int <= 200.0 {
        status.volume = vol_int;

        vlc::play( status.song.clone(), status.volume );

        data_store::save( &status );
    }

    return status;
}