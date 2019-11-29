use std::io;

use crate::view;
use crate::Status;


pub fn exec(status: &Status) {

    view::info(&status);

    let mut _s = String::new();
    io::stdin().read_line(&mut _s).expect("Did not enter a correct string");

}