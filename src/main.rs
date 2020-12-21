mod graphics_layers;
mod data_loader;

#[macro_use]
extern crate glium;


use log::{info, warn, debug};
use simple_logger::SimpleLogger;
use crate::data_loader::load_home_data;


fn main() {
    match SimpleLogger::new().init(){
        Ok(_) => debug!("Logger initialized"),
        Err(e) => panic!("Couldn't initialize basic logger. Error {}. Aborting", e)
    }

    // let home_data = load_home_data().expect("Couldn't query the API. Aborting");
    // for key in home_data.keys(){
    //     println!("{}", key.as_str());
    // }

    graphics_layers::launch_window();




}
