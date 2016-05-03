// crates
extern crate image;

// uses
use std::f64;
use image::RgbImage;
use image::imageops::*;

// mods
mod vec2;
use vec2::Vec2;

mod vec3;
use vec3::Vec3;

// vertices of WALLS in 2D
const WALLS: [(Vec2, Vec2); 17] =
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

// vertices of the corners of ROOMS in 2D
const ROOMS: [(Vec2, Vec2, Vec2, Vec2); 3] =
[
    (Vec2 { x:  0.0, y: 31.0 }, Vec2 { x: 96.0, y: 31.0 },
     Vec2 { x: 96.0, y:  0.0 }, Vec2 { x:  0.0, y:  0.0 }),
    (Vec2 { x:  7.0, y: 65.0 }, Vec2 { x: 77.0, y: 65.0 },
     Vec2 { x: 77.0, y: 31.0 }, Vec2 { x:  7.0, y: 31.0 }),
    (Vec2 { x:  0.0, y: 90.0 }, Vec2 { x: 77.0, y: 90.0 },
     Vec2 { x: 77.0, y: 65.0 }, Vec2 { x:  0.0, y: 65.0 })
];

// global variables
//const FREQUENCY:    f64 = 2.45;
const CEILING:      f64 = 2.9;
const ROOM_W:       f64 = 97.0;
const ROOM_H:       f64 = 91.0;
//const R_POW:        f64 = 13.0; // dBm
//const T_GAIN:       f64 = 2.0;  // dBi
//const R_GAIN:       f64 = 7.0;  // dBi
const N:            u32 = 50;
const T_POS: Vec3 = Vec3 { x: 39.0, y: 19.0, z: 0.45 };
const R_POS: Vec3 = Vec3 { x:  5.0, y: 25.0, z: 0.45 };
const COLOR_WALL:   [u8; 3] = [255, 255, 255];
const COLOR_T:      [u8; 3] = [255, 0, 0];
const COLOR_R:      [u8; 3] = [0, 0, 255];


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
    for wall in WALLS.iter() {
        let s1 = Vec3::new(wall.0.x, wall.0.y, CEILING);
        let s2 = Vec3::new(wall.1.x, wall.1.y, CEILING);
        let s3 = Vec3::new(wall.0.x, wall.0.y, 0.0);
        let s4 = Vec3::new(wall.1.x, wall.1.y, 0.0);
        planes.push((s1, s2, s3, s4));
    }
    for room in ROOMS.iter() {
        let floor =
        (
            Vec3::new(room.0.x, room.0.y, 0.0),
            Vec3::new(room.1.x, room.1.y, 0.0),
            Vec3::new(room.2.x, room.2.y, 0.0),
            Vec3::new(room.3.x, room.3.y, 0.0)
        );
        let roof =
        (
            Vec3::new(room.0.x, room.0.y, CEILING),
            Vec3::new(room.1.x, room.1.y, CEILING),
            Vec3::new(room.2.x, room.2.y, CEILING),
            Vec3::new(room.3.x, room.3.y, CEILING)
        );
        planes.push(floor);
        planes.push(roof);
    }

    // generate rays
    let mut rays = Vec::new();
    let angle_lat: f64 = 360.0 / (N as f64);
    let angle_lng: f64 = 360.0 / (N as f64);
    let mut lat: f64 = 0.0;
    let mut lng: f64 = 0.0;
    while lat < 360.0 {
        while lng < 360.0 {
            let t = f64::to_radians(lat);
            let s = f64::to_radians(lng);
            let x = T_POS.x + (1.0 * f64::cos(s) * f64::sin(t));
            let y = T_POS.y + (1.0 * f64::sin(s) * f64::sin(t));
            let z = T_POS.z + (1.0 * f64::cos(t));
            let ray =
            (
                Vec3::new(T_POS.x, T_POS.y, T_POS.z),
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
    let mut lowest = 10000.0;
    for ray in rays.iter() {
        let dist = Vec3::line_dist(&ray.0, &ray.1, &R_POS);
        if dist < lowest {
            lowest = dist;
        }
        //let cutoff = 200.0;
        //let mut distance = 0.0;
        //while distance < cutoff {
            // determine if the ray comes close enough to the r
            //if Vec3::line_dist(&ray.0, &ray.1, &R_POS) < 5.0 {
            //}
            // if it did, break and insert into top_rays if appropriate
            // else, determine the closest plane the ray intersects
            // then, determine the resulting ray after intersection
            // finally, sum the distance travelled
            //distance += 40.0;
        //}
    }
    println!("lowest: {}", lowest);

    // define image
    let mut image = RgbImage::new(ROOM_W as u32, ROOM_H as u32);

    // draw WALLS
    for wall in WALLS.iter() {
        let line = Vec2::line(&wall.0, &wall.1);
        for point in line.iter() {
            image.get_pixel_mut(point.x as u32, point.y as u32).data = COLOR_WALL;
        }
        image.get_pixel_mut(wall.0.x as u32, wall.0.y as u32).data = COLOR_WALL;
        image.get_pixel_mut(wall.1.x as u32, wall.1.y as u32).data = COLOR_WALL;
    }

    // draw transmitter/receiver
    image.get_pixel_mut(T_POS.x as u32, T_POS.y as u32).data = COLOR_T;
    image.get_pixel_mut(R_POS.x as u32, R_POS.y as u32).data = COLOR_R;

    for ray in rays.iter() {
        if ray.1.x > 0.0 && ray.1.x < ROOM_W && ray.1.y > 0.0 && ray.1.y < ROOM_H {
            image.get_pixel_mut(ray.1.x as u32, ray.1.y as u32).data = COLOR_R;
        }
    }

    // save image
    image = flip_vertical(&image);
    image.save("image.png").unwrap();
}
