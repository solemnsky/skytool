#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![allow(non_snake_case)]

#[macro_use] extern crate log;
extern crate serde;
extern crate serde_json;
extern crate libaltx;
extern crate zip;
extern crate math;

use math::Vec2d;
use std::fs::File;
use std::path::*;
use std::io::Result;
use libaltx::*;

#[derive(Debug,Serialize,Deserialize)]
pub struct Visuals{}
#[derive(Debug,Serialize,Deserialize)]
pub struct Mechanics{}

pub struct Environment{
    pub map: Option<Map>,
    pub visuals: Option<Visuals>,
    pub mechanics: Option<Mechanics>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Map{
    pub dimensions:  Vec2d,
    pub obstacles:   Vec<Obstacle>,
    pub spawnPoints: Vec<Spawn>,
    pub items:       Vec<Item>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Angle{ pub angle: f32 }

impl Angle{
    pub fn new(mut angle: f32) -> Angle{
        angle -= 90.;
        if angle < 0. { angle += 360.; }
        Angle{ angle: angle }
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Spawn{
    pub pos:   Vec2d,
    pub angle: Angle,
    pub team:  u16,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Item{
    pub pos: Vec2d,
    #[serde(rename="type")]
    pub kind: String,
}

#[allow(non_snake_case)]
#[derive(Debug,Serialize,Deserialize)]
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

        fn add_file<T: serde::Serialize>(zip: &mut zip::ZipWriter<File>,
                      name: &'static str, value: &T) -> Result<()>{
            try!(zip.start_file(name, Compress::Deflated));
            try!(write!(zip, "{}", serde_json::to_string_pretty(value).unwrap()));
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
    pub fn from_altx<P: AsRef<Path>>(path: P) -> Map{
        let mut f = File::open(path).unwrap();
        let a = archive::Archive::open(&mut f);
        let (name,s) = a.get_alte().unwrap();
        let map = map::Map::from_alte(name.to_owned(), s);
        Map::from_alt(&map)
    }

    pub fn from_alt(map: &libaltx::map::Map) -> Map{
        debug!("writing solemnsky map");
        for v in &map.views{
            if v.name == "Game"{
                let dimensions = Vec2d::new(v.size[0],v.size[1]);
                let mut obstacles = Vec::new();
                let mut spawn_points = Vec::new();
                for g in &v.geometry{
                    if g.collidable{
                        obstacles.push(Obstacle{
                            pos: Vec2d::new(g.pos[0],g.pos[1]),
                            localVertices:
                                g.hull.iter().map(|x| Vec2d::new(x[0],x[1]))
                                .collect(),
                            damage: 1,
                        });
                    }
                }
                for x in &v.spawn_points{
                    spawn_points.push(Spawn{
                        pos: Vec2d::new(x.pos[0], x.pos[1]),
                        angle: Angle::new(x.angle as f32),
                        team: x.team as u16,
                    });
                }
                return Map{
                    dimensions:  dimensions,
                    obstacles:   obstacles,
                    spawnPoints: spawn_points,
                    items:        Vec::new(),
                };
            }
        }
        unimplemented!();
    }
}
