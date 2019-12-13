use crate::vlc_wrapper as vlc;
use crate::Status;


pub fn exec(mut status: Status) -> Status {

    status.song = "|||  ".split("|||").map(|s| s.to_string()).collect();

    vlc::stop();

    return status;
}