#![feature(plugin)]
#![plugin(docopt_macros)]

#[macro_use] extern crate log;
extern crate logger;
extern crate sky;
extern crate rustc_serialize;
extern crate docopt;

use std::path::Path;
use std::fs;
use sky::*;

docopt!(Args derive Debug, "
Usage:
  skytool convert <file> [-o <directory>]
  skytool (-h | --help)

Options:
  -h --help     Show this screen.
  -o --output <directory>  The directory in which to save files [default: ./]
");

fn main(){
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    logger::init();

    if args.cmd_convert {
        fs::create_dir_all(&args.flag_output)
            .expect("Failed to create output directory");
        let path = Path::new(&args.arg_file);
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if file_name.ends_with(".altx") {

            let map_name = &file_name[0..file_name.len()-5];
            let out = Path::new(&args.flag_output)
                .join(&format!("{}.sky", map_name));

            info!("Converting {} to {:?}", map_name, out);

            let mut env = Environment::new();
            env.map = Some(Map::from_altx(map_name));

            env.to_sky(&out).unwrap();
        } else {
            error!("Only altx -> sky converion is implemented for now.");
        }
    }
}
