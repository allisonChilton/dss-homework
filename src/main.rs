mod graphics_layers;
mod data_loader;

#[macro_use]
extern crate glium;


use log::{debug};
use simple_logger::SimpleLogger;
use crate::data_loader::{load_home_data, get_image_cache};


fn main() {
    match SimpleLogger::new().init(){
        Ok(_) => debug!("Logger initialized"),
        Err(e) => panic!("Couldn't initialize basic logger. Error {}. Aborting", e)
    }

    let home_data = load_home_data().expect("Couldn't query the API. Aborting");
    let img_cache = get_image_cache(&home_data);
    // let home_data = Vec::new();

    graphics_layers::launch_window(home_data, img_cache);

}
