// crates
extern crate image;

// uses
use std::f64;
use std::cmp::Ordering;
use image::RgbImage;
use image::imageops::*;

// mods
mod vec2;
use vec2::Vec2;

mod vec3;
use vec3::Vec3;

mod signal;
use signal::Signal;

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
const FREQUENCY:    f64 = 2.45;
const CEILING:      f64 = 2.9;
const ROOM_W:       f64 = 97.0;
const ROOM_H:       f64 = 91.0;
const T_POW:        f64 = 13.0; // dBm
const T_GAIN:       f64 = 2.0;  // dBi
const R_GAIN:       f64 = 7.0;  // dBi
const N:            u32 = 100;
const T_POS: Vec3 = Vec3 { x: 39.0, y: 19.0, z: 0.45 }; 
const COLOR_WALL:   [u8; 3] = [255, 255, 255];
const COLOR_T:      [u8; 3] = [255, 0, 0];
const COLOR_R:      [u8; 3] = [0, 0, 255];

fn ray_plane_isect(v1: &Vec3, v2: &Vec3, plane: &(Vec3, Vec3, Vec3, Vec3)) -> Option<Vec3> {
    let dS21 = Vec3::sub(&plane.1, &plane.0);
    let dS31 = Vec3::sub(&plane.2, &plane.0);
    let n = Vec3::cross(&dS21, &dS31);
    let dR = Vec3::sub(&v1, &v2);
    let ndotdR = Vec3::dot(&n, &dR);
    if ndotdR.abs() < 1e-6 {
        return None;
    }
    let t = Vec3::dot(&Vec3::mul(&n, -1.0), &Vec3::sub(&v1, &plane.0)) / ndotdR;
    let M = Vec3::add(&v1, &Vec3::mul(&dR, t));
    let dMS1 = Vec3::sub(&M, &plane.0);
    let u = Vec3::dot(&dMS1, &dS21);
    let v = Vec3::dot(&dMS1, &dS31);
    if u >= 0.0 && u <= Vec3::dot(&dS21, &dS21) 
    && v >= 0.0 && v <= Vec3::dot(&dS31, &dS31) {
        return Some(M);
    } else {
        None
    }
}

fn ray_plane_angle(v: &Vec3, plane: &(Vec3, Vec3, Vec3, Vec3)) -> f64 {
    let normal = Vec3::plane_norm(&plane);
    let mut angle = f64::atan2(v.x, v.y) - f64::atan2(normal.x, normal.y);
    if angle < 0.0 {
        angle += 2.0 * f64::consts::PI;
    }
    angle
}

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

    // define image
    let mut image = RgbImage::new(ROOM_W as u32, ROOM_H as u32);
   
    let cutoff = 200.0;
    let threshold = 1.0;

    for x in 0..ROOM_W as u32 {
        for y in 0..ROOM_H as u32 { 
            let r_pos = Vec3::new(x as f64, y as f64, 0.45);
            let mut best_rays = Vec::new();
            for ray in rays.iter() { 
                let mut distance = 0.0;
                let mut iterations = 0;
                let mut temp_ray = ray.clone();
                // find the closest point of intersection with a plane
                let mut closest_isect: (Vec3, f64, usize) = (Vec3::new(0.0, 0.0, 0.0), cutoff, 0);
                while distance < cutoff && iterations < 20 {
                    for (i, plane) in planes.iter().enumerate() {
                        match ray_plane_isect(&temp_ray.0, &temp_ray.1, &plane) {
                            Some(point) => {
                                let dist = Vec3::dist(&temp_ray.0, &point);
                                if dist < closest_isect.1 {
                                    closest_isect.0 = point.clone();
                                    closest_isect.2 = i;
                                }
                            },
                            _ => {}
                        }
                    }
                    if closest_isect.0 == Vec3::new(0.0, 0.0, 0.0) {
                        break;
                    } else {
                        temp_ray.1 = closest_isect.0.clone();
                    }
                    // check if line is close enough
                    let line_dist = Vec3::line_dist(&temp_ray.0, &temp_ray.1, &r_pos);
                    if  line_dist < threshold {
                        if best_rays.len() < 3 { 
                            best_rays.push((temp_ray, line_dist));
                            break;
                        } else {
                            best_rays.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
                            best_rays.pop();
                            best_rays.push((temp_ray, line_dist));
                            break; 
                        }
                    } 
                    // else, calculate resulting ray and increment dist
                    distance += Vec3::dist(&temp_ray.0, &closest_isect.0); 
                    let norm = Vec3::vec_norm(&Vec3::plane_norm(&planes[closest_isect.2]));
                    let dot = Vec3::dot(&temp_ray.0, &norm);
                    let refl = Vec3::mul(&Vec3::mul(&norm, dot), -2.0);
                    temp_ray.0 = closest_isect.0.clone();
                    temp_ray.1 = Vec3::sub(&temp_ray.0, &refl);
                    iterations += 1;
                }
            }
            println!("{}", best_rays.len());
        }
    }

    // draw WALLS
    for wall in WALLS.iter() {
        let line = Vec2::line(&wall.0, &wall.1);
        for point in line.iter() {
            image.get_pixel_mut(point.x as u32, point.y as u32).data = COLOR_WALL;
        }
        image.get_pixel_mut(wall.0.x as u32, wall.0.y as u32).data = COLOR_WALL;
        image.get_pixel_mut(wall.1.x as u32, wall.1.y as u32).data = COLOR_WALL;
    }

    // save image
    image = flip_vertical(&image);
    image.save("image.png").unwrap();
}
