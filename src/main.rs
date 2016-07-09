#[macro_use] extern crate log;
extern crate rustc_serialize;
extern crate logger;
extern crate libaltx;

use std::fs::{self,File};
use std::io::Write;
use libaltx::map::Map;
use rustc_serialize::json;

fn main(){
    logger::init();

    for f in fs::read_dir("maps").unwrap(){
        let f = f.unwrap();
        let file_name = f.file_name().into_string().unwrap();
        if file_name.ends_with(".altx") {
            let map_name = &file_name[0..file_name.len()-5];
            let mut map = Map::from_altx(map_name);
            map.clear_geom();

            let mut file = File::create(format!("out/{}.map", map_name)).unwrap();
            write!(file, "{}", json::encode(&map).unwrap()).unwrap();
        }
    }
}
