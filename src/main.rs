#![feature(plugin)]
#![plugin(docopt_macros)]

#[macro_use] extern crate log;
extern crate rustc_serialize;
extern crate logger;
extern crate libaltx;
extern crate docopt;

use std::path::Path;
use libaltx::solemnsky::*;

docopt!(Args derive Debug, "
Usage:
  skytool convert <file>
  skytool (-h | --help)

Options:
  -h --help     Show this screen.
");

fn main(){
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    logger::init();

    info!("{:?}", args);

    if args.cmd_convert {
        let path = Path::new(&args.arg_file);
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".altx") {
            let map_name = &file_name[0..file_name.len()-5];
            let out = format!("{}.sky", map_name);
            info!("Converting {} to {}",file_name,out);

            let mut env = Environment::new();
            env.map = Some(Map::from_alt(
                    &libaltx::map::Map::from_altx(map_name)));

            env.to_sky(&out).unwrap();
        } else {
            error!("Only altx -> sky converion is implemented for now.");
        }
    }
}
