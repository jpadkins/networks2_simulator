extern crate image;

use std::io::Read;
use std::fs::File;
use std::mem;
use image::RgbImage;
use image::imageops::*;

mod vec2;
use vec2::Vec2;

mod vec3;
use vec3::Vec3;

const walls: [(Vec2, Vec2); 17] = 
[
    (Vec2 { x:  0.0, y:  0.0 }, Vec2 { x: 96.0, y:  0.0 }),
    (Vec2 { x: 77.0, y:  0.0 }, Vec2 { x: 77.0, y: 15.0 }),
    (Vec2 { x:  0.0, y:  0.0 }, Vec2 { x:  0.0, y: 31.0 }),
    (Vec2 { x: 96.0, y:  0.0 }, Vec2 { x: 96.0, y: 31.0 }),
    (Vec2 { x:  0.0, y: 31.0 }, Vec2 { x: 30.0, y: 31.0 }),
    (Vec2 { x: 41.0, y: 31.0 }, Vec2 { x: 96.0, y: 31.0 }),
    (Vec2 { x: 77.0, y: 31.0 }, Vec2 { x: 77.0, y: 96.0 }),
    (Vec2 { x:  0.0, y: 90.0 }, Vec2 { x: 77.0, y: 90.0 }),
    (Vec2 { x:  7.0, y: 31.0 }, Vec2 { x:  7.0, y: 65.0 }),
    (Vec2 { x:  0.0, y: 65.0 }, Vec2 { x: 30.0, y: 65.0 }),
    (Vec2 { x:  7.0, y: 48.0 }, Vec2 { x: 30.0, y: 48.0 }),
    (Vec2 { x:  0.0, y: 65.0 }, Vec2 { x:  0.0, y: 90.0 }),
    (Vec2 { x: 41.0, y: 31.0 }, Vec2 { x: 41.0, y: 52.0 }),
    (Vec2 { x: 41.0, y: 76.0 }, Vec2 { x: 41.0, y: 90.0 }),
    (Vec2 { x: 41.0, y: 65.0 }, Vec2 { x: 77.0, y: 65.0 }),
    (Vec2 { x: 30.0, y: 43.0 }, Vec2 { x: 30.0, y: 52.0 }), 
    (Vec2 { x: 31.0, y: 76.0 }, Vec2 { x: 41.0, y: 76.0 }),
];

const rooms: [(Vec2, Vec2, Vec2, Vec2); 3] =
[
    (Vec2 { x:  0.0, y: 31.0 }, Vec2 { x: 96.0, y: 31.0 },
     Vec2 { x: 96.0, y:  0.0 }, Vec2 { x:  0.0, y:  0.0 }),
    (Vec2 { x:  7.0, y: 65.0 }, Vec2 { x: 77.0, y: 65.0 },
     Vec2 { x: 77.0, y: 31.0 }, Vec2 { x:  7.0, y: 31.0 }),
    (Vec2 { x:  0.0, y: 90.0 }, Vec2 { x: 77.0, y: 90.0 },
     Vec2 { x: 77.0, y: 65.0 }, Vec2 { x:  0.0, y: 65.0 })
];

const frequency:    f64 = 2.45;
const ceiling:      f64 = 2.9;
const room_w:       f64 = 97.0;
const room_h:       f64 = 91.0;
const t_gain:       f64 = 20.0;
const r_gain:       f64 = 20.0;
const t_pos: Vec3 = Vec3 { x: 39.0, y: 19.0, z: 1.4 };
const r_pos: Vec3 = Vec3 { x:  5.0, y: 25.0, z: 1.4 };

fn main() {
    let mut image = RgbImage::new(room_w as u32, room_h as u32);
    for wall in walls.iter() {
        
    }
}
