#![allow(warnings, unused)]
mod graphics_layers;
mod data_loader;

#[macro_use]
extern crate glium;


use log::{info, warn, debug};
use simple_logger::SimpleLogger;
use crate::data_loader::load_home_data;
use graphics_layers::Rectangle;


fn main() {
    match SimpleLogger::new().init(){
        Ok(_) => debug!("Logger initialized"),
        Err(e) => panic!("Couldn't initialize basic logger. Error {}. Aborting", e)
    }

    let home_data = load_home_data().expect("Couldn't query the API. Aborting");
    // let home_data = Vec::new();

    graphics_layers::launch_window(home_data);

}
