use std::io;
use crossterm::{ExecutableCommand, terminal};
// use std::process;

mod data_store;
mod view;

mod vlc_wrapper;
use vlc_wrapper as vlc;

mod menu;
use menu::menu as ui;


#[derive(Clone)]
pub struct Status {
    volume: f32,
    song: Vec<String>,
    playing: bool,

    show_all: i32,
    songs: Vec<String>
}


fn main() {

    setup_checks();

    // just to set TERM flag ...
    io::stdout().execute(terminal::Clear(terminal::ClearType::All)).expect("Something went getting the handle");
    
    vlc::play_handeler_setup();

    ui::init();
    
}

fn setup_checks() {

    data_store::make_data_store_valid();
    
    // if !vlc::path_is_set() {
    //     eprintln!("\n vlc path is not set.");
    //     process::exit(0);
    // } else {
    //     vlc::stop();
    // }

}