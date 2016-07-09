#[macro_use] extern crate log;
extern crate rustc_serialize;
extern crate libaltx;
extern crate zip;
extern crate math;

use math::Vec2d;
use std::fs::File;
use std::path::*;
use std::io::Result;
use rustc_serialize::*;

#[derive(Debug,RustcDecodable,RustcEncodable)]
pub struct Visuals{}
#[derive(Debug,RustcDecodable,RustcEncodable)]
pub struct Mechanics{}

pub struct Environment{
    pub map: Option<Map>,
    pub visuals: Option<Visuals>,
    pub mechanics: Option<Mechanics>,
}

#[allow(non_snake_case)]
#[derive(Debug,RustcDecodable,RustcEncodable)]
pub struct Map{
    pub dimensions:  Vec2d,
    pub obstacles:   Vec<Obstacle>,
    pub spawnPoints: Vec<Spawn>,
}

#[derive(Debug,RustcDecodable,RustcEncodable)]
pub struct Angle{ pub angle: f32 }

impl Angle{
    pub fn new(mut angle: f32) -> Angle{
        angle -= 90.;
        if angle < 0. { angle += 360.; }
        Angle{ angle: angle }
    }
}

#[derive(Debug,RustcDecodable,RustcEncodable)]
pub struct Spawn{
    pub pos:   Vec2d,
    pub angle: Angle,
    pub team:  u16,
}

#[allow(non_snake_case)]
#[derive(Debug,RustcDecodable,RustcEncodable)]
pub struct Obstacle{
    pub pos: Vec2d,
    pub localVertices: Vec<Vec2d>,
    pub damage: i32,
}

impl Environment{
    pub fn new() -> Environment{
        Environment{
            map: None,
            visuals: Some(Visuals{}),
            mechanics: Some(Mechanics{}),
        }
    }

    pub fn to_sky(&self, path: &PathBuf) -> Result<()>{
        use std::io::Write;
        use zip::CompressionMethod as Compress;
        let file = try!(File::create(path));
        let mut zip = zip::ZipWriter::new(file);

        fn add_file<T: Encodable>(zip: &mut zip::ZipWriter<File>,
                      name: &'static str, value: &T) -> Result<()>{
            try!(zip.start_file(name, Compress::Deflated));
            try!(write!(zip, "{}", json::encode(value).unwrap()));
            Ok(())
        };

        if let Some(ref x) = self.map {
            try!(add_file(&mut zip, "map.json", x)); }
        if let Some(ref x) = self.visuals {
            try!(add_file(&mut zip, "visuals.json", x)); }
        if let Some(ref x) = self.mechanics {
            try!(add_file(&mut zip, "mechanics.json", x)); }

        Ok(())
    }
}

impl Map{
    pub fn from_altx(name: &str) -> Map{
        Map::from_alt(&libaltx::map::Map::from_altx(name))
    }

    pub fn from_alt(map: &libaltx::map::Map) -> Map{
        debug!("writing solemnsky map");
        for v in &map.views{
            if v.name == "Game"{
                let dimensions = v.bounds[1];
                let mut obstacles = Vec::new();
                let mut spawn_points = Vec::new();
                for g in &v.geometry{
                    if g.collidable{
                        obstacles.push(Obstacle{
                            pos: g.pos,
                            localVertices: g.hull.clone(),
                            damage: 1,
                        });
                    }
                }
                let mut main_team: i32 = -1;
                for i in 0..v.spawn_points.len(){
                    for s in &v.spawn_points[i]{
                        if main_team == -1 { main_team = i as i32; }
                        let team = if i as i32 == main_team { 1 } else { 2 };
                        spawn_points.push(Spawn{
                            pos: s.0, angle: Angle::new(s.1.round()), team: team as u16 });
                    }
                }
                return Map{
                    dimensions:  dimensions,
                    obstacles:   obstacles,
                    spawnPoints: spawn_points,
                };
            }
        }
        unimplemented!();
    }
}
