use std::io;
use std::fs;

use super::options;
use crate::view;
use crate::Status;


pub fn init() {

    let vol_int: f32;
    let contents = fs::read_to_string("./.CliRr")
        .expect("Something went wrong reading the file");
    let mut lines = contents.lines();

    let vol = lines.next().unwrap().to_string();

    match vol.parse::<f32>() {
        Ok(n) => vol_int = n,
        Err(_e) => vol_int = 100.0
    }

    let mut status = Status {
        volume: vol_int,
        song: "|||  ".split("|||").map(|s| s.to_string()).collect(),
        playing: false,

        show_all: -1,
        songs: lines.map(|s| s.to_string()).collect()
    };

    loop {
        status = mloop(status);
    }

}

fn mloop(mut status: Status) -> Status {
    
    view::display(&status);


    let mut input = String::new();
    {
        io::stdin().read_line(&mut input).expect("Did not enter a correct string");
        if let Some('\n')=input.chars().next_back() { input.pop(); }
        if let Some('\r')=input.chars().next_back() { input.pop(); }
    }


    if input.starts_with("a") {

        status = options::show_all::exec(status, input);

    } else if input.starts_with("v") {

        status = options::change_volume::exec(status, input);

    } else if input.starts_with("n") {

        status = options::add_new::exec(status, input);

    }  else if input == "i" {

        options::info::exec(&status);

    } else if input == "q" {

        options::quit::exec();

    } else if input == "s" {

        status = options::stop::exec(status);

    } else {

        status = options::play::exec(status, input);

    }


    return status;
}