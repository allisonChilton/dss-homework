mod graphics_layers;

#[macro_use]
extern crate glium;

use std::fs::File;
use std::env;
use log::{info, warn, debug};
use simple_logger::SimpleLogger;
use reqwest::blocking::Response;
use reqwest::Error;
use std::collections::HashMap;

fn load_home_data() -> reqwest::Result<HashMap<String, String>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?;
    resp.json::<HashMap<String, String>>()
}

fn main() {
    match SimpleLogger::new().init(){
        Ok(e) => debug!("Logger initialized"),
        Err(e) => panic!("Couldn't initialize basic logger. Error {}. Aborting", e)
    }

    let home_data = load_home_data().expect("Couldn't query the API. Aborting");
    for key in home_data.keys(){
        println!("{}", key.as_str());
    }

    graphics_layers::launch_window();




}
