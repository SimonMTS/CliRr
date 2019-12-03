use std::io;

use super::options;
use crate::view;
use crate::Status;
use crate::data_store;


pub fn init() {

    let mut status = data_store::read();

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