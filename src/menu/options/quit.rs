use std::process;

use crate::vlc_wrapper as vlc;


pub fn exec() {

    vlc::stop();
    process::exit(0);

}