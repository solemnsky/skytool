extern crate libaltx;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate colored;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rustc_serialize;

use rustc_serialize::json;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use libaltx::map::Map;
use libaltx::solemnsky::SolemnskyMap;
use colored::*;
use log::{LogRecord,LogLevelFilter};
use env_logger::LogBuilder;
use std::env;
use std::thread;
use std::fs::File;
use std::io::Write;

fn main(){
    init_logger();

    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "alti-map",
            [1000, 600]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let map = Map::from_altx("ball_cloud_pb");

    let solemnsky = SolemnskyMap::from_alt(&map);
    let mut file = File::create("roids.json").unwrap();
    write!(file, "{}", json::encode(&solemnsky).unwrap()).unwrap();


    let mut gl = GlGraphics::new(opengl);

    let scale = 0.5f64;
    //let scale = 1.0f64;

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            use graphics::*;

            const BG: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
            let mut fg = [0.5, 0.5, 0.5, 0.5];
            let outline = [1.0, 1.0, 1.0, 1.0];
            let mut col = 0;

            gl.draw(r.viewport(), |c, gl| {
                clear(BG, gl);

                let mut counter = 1;
                for layer in &map.views{
                    //if layer.name == "Game"{
                        for g in &layer.geometry{
                            if g.visible && g.collidable{
                                for i in 0..2{
                                    if i == col {fg[i] += 0.02;}
                                    else {fg[i] -= 0.01;}
                                }
                                if counter % 30 == 0{
                                    col += 1;
                                    if col > 2 { col = 0; }
                                }
                                let mut poly = Vec::new();
                                for p in &g.hull{ poly.push([p.x as f64,p.y as f64]); }
                                let mut lines = Vec::new();

                                for i in 1..poly.len(){
                                    let p1 = poly[i-1]; let p2 = poly[i];
                                    lines.push([p1[0],p1[1],p2[0],p2[1]]);
                                }
                                let p1 = poly[0]; let p2 = poly[poly.len()-1];
                                lines.push([p1[0],p1[1],p2[0],p2[1]]);

                                let line = Line::new(outline,1.0);

                                let transform = c.transform
                                    .trans(scale * g.pos.x as f64, scale * g.pos.y as f64)
                                    .scale(scale,scale);
                                polygon(fg, &poly, transform, gl);

                                for l in lines{
                                    line.draw(l, &c.draw_state, transform, gl);
                                }

                                counter += 1;
                            }
                        }
                    //}
                }

            });
        }
        if let Some(u) = e.update_args() { }
    }



}

fn init_logger(){
    let format = |record: &LogRecord| {
        use log::LogLevel::*;
        let location = record.location().module_path();
        let location = match record.level(){
            Error => location.red(),
            Warn => location.yellow(),
            Info => location.green().dimmed(),
            Debug => location.cyan().dimmed(),
            Trace => location.blue().dimmed(),
        };
        format!("{:15} {}", location, record.args())
    };
    let mut builder = LogBuilder::new();
    builder.format(format).filter(None, LogLevelFilter::Info);
    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    }
    builder.init().unwrap();
}
