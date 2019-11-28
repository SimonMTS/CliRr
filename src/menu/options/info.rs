use std::io;

use crate::view;
use crate::Status;


pub fn exec(status: &Status) {

    view::header(&status);

    println!(" [a] show All:\n   Shows the full list of stored songs.\n");
    println!(" [n] add New: \n   \x1B[1m\"n <video_id>\"\x1B[0m, adds the youtube video_id to the top of the list, and starts playing it.\n");

    println!(" <Press enter to continue>");

    let mut _s = String::new();
    io::stdin().read_line(&mut _s).expect("Did not enter a correct string");

}