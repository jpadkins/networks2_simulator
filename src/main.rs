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
    (Vec2 { x: 77.0, y: 31.0 }, Vec2 { x: 77.0, y: 90.0 }),
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

const color_wall: [u8; 3] = [255, 255, 255];

fn main() {
    let mut image = RgbImage::new(room_w as u32, room_h as u32);
    for (i, wall) in walls.iter().enumerate() {
        let line = Vec2::line(&wall.0, &wall.1);
        for point in line.iter() {
            image.get_pixel_mut(point.x as u32, point.y as u32).data = color_wall;
        }
        image.get_pixel_mut(wall.0.x as u32, wall.0.y as u32).data = color_wall;
        image.get_pixel_mut(wall.1.x as u32, wall.1.y as u32).data = color_wall;
    } 
    image = flip_vertical(&image);
    image.save("image.png").unwrap();
}
