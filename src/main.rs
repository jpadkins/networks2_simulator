// crates
extern crate image;

// uses
use std::io::Read;
use std::fs::File;
use std::f64;
use std::mem;
use image::RgbImage;
use image::imageops::*;

// mods
mod vec2;
use vec2::Vec2;

mod vec3;
use vec3::Vec3;

// vertices of walls in 2D
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

// vertices of the corners of rooms in 2D
const rooms: [(Vec2, Vec2, Vec2, Vec2); 3] =
[
    (Vec2 { x:  0.0, y: 31.0 }, Vec2 { x: 96.0, y: 31.0 },
     Vec2 { x: 96.0, y:  0.0 }, Vec2 { x:  0.0, y:  0.0 }),
    (Vec2 { x:  7.0, y: 65.0 }, Vec2 { x: 77.0, y: 65.0 },
     Vec2 { x: 77.0, y: 31.0 }, Vec2 { x:  7.0, y: 31.0 }),
    (Vec2 { x:  0.0, y: 90.0 }, Vec2 { x: 77.0, y: 90.0 },
     Vec2 { x: 77.0, y: 65.0 }, Vec2 { x:  0.0, y: 65.0 })
];

// global variables
const frequency:    f64 = 2.45;
const ceiling:      f64 = 2.9;
const room_w:       f64 = 97.0;
const room_h:       f64 = 91.0;
const t_pow:        f64 = 13.0; // dBm
const t_gain:       f64 = 2.0;  // dBi
const r_gain:       f64 = 7.0;  // dBi
const n:            u32 = 50;
const t_pos: Vec3 = Vec3 { x: 39.0, y: 19.0, z: 0.45 };
const r_pos: Vec3 = Vec3 { x:  5.0, y: 25.0, z: 0.45 }; 
const color_wall:   [u8; 3] = [255, 255, 255];
const color_t:      [u8; 3] = [255, 0, 0];
const color_r:      [u8; 3] = [0, 0, 255];


fn main() {
    /*
     * planes are defined by points:
      S1 +------+ S2
         |      |
         |      |
      S3 +------+ S4
    */
    let mut planes = Vec::new();

    // populate planes
    for wall in walls.iter() {
        let s1 = Vec3::new(wall.0.x, wall.0.y, ceiling);
        let s2 = Vec3::new(wall.1.x, wall.1.y, ceiling);
        let s3 = Vec3::new(wall.0.x, wall.0.y, 0.0);
        let s4 = Vec3::new(wall.1.x, wall.1.y, 0.0);
        planes.push((s1, s2, s3, s4));
    }
    for room in rooms.iter() {
        let floor =
        (
            Vec3::new(room.0.x, room.0.y, 0.0),
            Vec3::new(room.1.x, room.1.y, 0.0),
            Vec3::new(room.2.x, room.2.y, 0.0),
            Vec3::new(room.3.x, room.3.y, 0.0)
        );
        let roof =
        (
            Vec3::new(room.0.x, room.0.y, ceiling),
            Vec3::new(room.1.x, room.1.y, ceiling),
            Vec3::new(room.2.x, room.2.y, ceiling),
            Vec3::new(room.3.x, room.3.y, ceiling)
        );
        planes.push(floor);
        planes.push(roof);
    }

    // generate rays
    let mut rays = Vec::new();
    let angle_lat: f64 = 360.0 / (n as f64);
    let angle_lng: f64 = 360.0 / (n as f64);
    let mut lat: f64 = 0.0;
    let mut lng: f64 = 0.0;
    while lat < 360.0 {
        while lng < 360.0 {
            let t = f64::to_radians(lat);
            let s = f64::to_radians(lng);
            let x = t_pos.x + 1.0 * f64::cos(s) * f64::sin(t);
            let y = t_pos.y + 1.0 * f64::sin(s) * f64::sin(t);
            let z = t_pos.z + 1.0 * f64::cos(t);
            let ray =
            (
                Vec3::new(t_pos.x, t_pos.y, t_pos.z),
                Vec3::new(x, y, z)
            ); 
            rays.push(ray);
            lng += angle_lng;
        }
        lng = 0.0;
        lat += angle_lat; 
    }

    // trace each ray, tracking top three 
    //let top_rays = Vec::new();
    for ray in rays.iter() {
        let mut cutoff = 200.0;
        let mut distance = 0.0;
        while distance < cutoff { 
            // determine if the ray comes close enough to the r
            // if it did, break and insert into top_rays if appropriate
            // else, determine the closest plane the ray intersects
            // then, determine the resulting ray after intersection
            // finally, sum the distance travelled
            distance += 40.0;
        }
    } 

    // define image
    let mut image = RgbImage::new(room_w as u32, room_h as u32);

    // draw walls
    for wall in walls.iter() {
        let line = Vec2::line(&wall.0, &wall.1);
        for point in line.iter() {
            image.get_pixel_mut(point.x as u32, point.y as u32).data = color_wall;
        }
        image.get_pixel_mut(wall.0.x as u32, wall.0.y as u32).data = color_wall;
        image.get_pixel_mut(wall.1.x as u32, wall.1.y as u32).data = color_wall;
    }
    
    // draw transmitter/receiver
    image.get_pixel_mut(t_pos.x as u32, t_pos.y as u32).data = color_t;
    image.get_pixel_mut(r_pos.x as u32, r_pos.y as u32).data = color_r;

    for ray in rays.iter() {
        image.get_pixel_mut(ray.1.x as u32, ray.1.y as u32).data = color_r;   
    }
    
    // save image
    image = flip_vertical(&image);
    image.save("image.png").unwrap();
}
