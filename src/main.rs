// crates
extern crate image;

// inclusions
use std::fmt;
use std::io::Read;
use std::fs::File; 
use std::collections::HashMap;
use image::RgbImage;
use image::imageops::*;

#[derive(PartialEq, PartialOrd, Clone)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Vec2 {
        Vec2 {
            x: x,
            y: y,
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(PartialEq, PartialOrd, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z,
        }
    }
    fn change_pos(&mut self, x: f64, y: f64, z: f64) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
    fn add(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3::new(v1.x+v2.x, v1.y+v2.y, v1.z+v2.z)
    }
    fn sub(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3::new(v1.x-v2.x, v1.y-v2.y, v1.z-v2.z)
    }
    fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
    }
    fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3::new(v1.y * v2.z - v1.z * v2.y,
                  v1.z - v2.x - v1.x * v2.z,
                  v1.x - v2.y - v1.y * v2.x)
    }
    fn mul(v: &Vec3, f: f64) -> Vec3 {
        Vec3::new(v.x*f, v.y*f, v.z*f)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

fn retrieve(config: &HashMap<&str, f64>, key: &str) -> f64 { 
    match config.get(key) {
        Some(value) => { return *value },
        None => { panic!("No value in config for <{}>!", key); }
    } 
}

fn main() {
    println!("\n
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//
    //    Welcome to The Jacob Adkins Multipath Simulator!   // 
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//
    ");
    
    println!("
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Stage 1: Parse Config and Room Files
    ");
    // read config file
    print!("Reading config file. . . .");
    let mut config_str = String::new();
    let mut config_f = File::open("data/config.txt").unwrap();
    config_f.read_to_string(&mut config_str).unwrap();
    println!("Okay!");

    // read room file
    print!("Reading room file. . . .");
    let mut room_str = String::new();
    let mut room_f = File::open("data/room.txt").unwrap();
    room_f.read_to_string(&mut room_str).unwrap();
    println!("Okay!");

    // declare config hashmap and vars
    let mut config: HashMap<&str, f64> = HashMap::new();
    let mut t_pos = Vec3::new(0_f64, 0_f64, 0_f64);
    let mut r_pos = Vec3::new(0_f64, 0_f64, 0_f64);

    // declare room vector
    let mut room: Vec<(Vec2, Vec2)> = Vec::new(); 

    // parse config
    println!("\nParsing config file contents:");
    let lines = config_str.split('\n');
    for line in lines {
        let mut key_value = line.split(": ");
        match (key_value.next(), key_value.next()) {
            (Some(key), Some(val)) => {
                if val.chars().nth(0).unwrap() == '(' {
                    let chars_to_trim: &[char] = &['(', ')'];
                    let trimmed_val = val.trim_matches(chars_to_trim);
                    let mut xyz = trimmed_val.split(", ");
                    match (xyz.next(), xyz.next(), xyz.next()) {
                        (Some(x), Some(y), Some(z)) => { 
                            let x_f64 = x.parse::<f64>().unwrap();
                            let y_f64 = y.parse::<f64>().unwrap();
                            let z_f64 = z.parse::<f64>().unwrap();
                            let point = Vec3::new(x_f64, y_f64, z_f64); 
                            if key == "t_pos" {
                                t_pos = point;
                            } else if key == "r_pos" {
                                r_pos = point;
                            } else {
                                panic!("malformed config file!");
                            }
                        }
                        _ => {}
                    } 
                } else {
                    let val_f64 = val.parse::<f64>().unwrap(); 
                    config.insert(key, val_f64);
                } 
            },
            _ => {}
        }
    }
    println!(". . . .Done!");

    // parse room
    println!("\nParsing room file contents:");
    let lines = room_str.split('\n');
    for line in lines {
        let mut p1p2 = line.split("), (");
        match (p1p2.next(), p1p2.next()) {
            (Some(p1), Some(p2)) => {
                let chars_to_trim: &[char] = &['(', ')'];
                let trimmed_p1 = p1.trim_matches(chars_to_trim);
                let trimmed_p2 = p2.trim_matches(chars_to_trim);
                let mut xy1 = trimmed_p1.split(", ");
                let mut xy2 = trimmed_p2.split(", ");
                match (xy1.next(), xy1.next(), xy2.next(), xy2.next()) {
                    (Some(x1), Some(y1), Some(x2), Some(y2)) => {
                        let x1_f64 = x1.parse::<f64>().unwrap();
                        let y1_f64 = y1.parse::<f64>().unwrap();
                        let x2_f64 = x2.parse::<f64>().unwrap();
                        let y2_f64 = y2.parse::<f64>().unwrap();
                        let point1 = Vec2::new(x1_f64, y1_f64);
                        let point2 = Vec2::new(x2_f64, y2_f64); 
                        room.push((point1, point2));
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
    println!(". . . .Done!");

    // print out parsed values
    println!("\nParsed config values:");
    println!("Transmitter position:\t{}", t_pos);
    println!("Receiver Position:\t{}", r_pos);
    println!("Frequency:\t\t{}", retrieve(&config, "frequency"));
    println!("Ceiling:\t\t{}", retrieve(&config, "ceiling"));
    println!("Room Width:\t\t{}", retrieve(&config, "room_w"));
    println!("Room Height:\t\t{}", retrieve(&config, "room_h"));
    println!("Transmitter Gain:\t{}", retrieve(&config, "t_gain"));
    println!("Receiver Gain:\t\t{}", retrieve(&config, "r_gain"));
    println!("Transmitter Angle:\t{}", retrieve(&config, "t_angle"));

    // print out parsed room wall definitions
    println!("\nParsed room wall definitions:");
    for (i, wall) in room.iter().enumerate() {
        println!("Wall #{}: {} - {}", i, wall.0, wall.1);
    }

    println!("
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Stage 2: Contruct Virtual Indoor Space
    ");
    
    let mut image = RgbImage::new(retrieve(&config, "room_w") as u32, retrieve(&config, "room_h") as u32); 
    
    for wall in room.iter() {
        image.get_pixel_mut(wall.0.x as u32, wall.0.y as u32).data = [255, 255, 255];
        let mut x1 = wall.0.x as u32;
        let mut y1 = wall.0.y as u32;
        let x2 = wall.1.x as u32;
        let y2 = wall.1.y as u32;
        image.get_pixel_mut(x1, y1).data = [255, 255, 255];
        loop { 
            if x1 > x2 { x1 -= 1; }
            if x1 < x2 { x1 += 1; }
            if y1 > y2 { y1 -= 1; }
            if y1 < y2 { y1 += 1; }
            image.get_pixel_mut(x1, y1).data = [255, 255, 255];
            if x1 == x2 && y1 == y2 { break; }
        } 
    }

    // generate the rays
    
    let mut ray = t_pos;
    for _i in 1..20 {
        image.get_pixel_mut(ray.x as u32, ray.y as u32).data = [255, 0, 0];                
        ray.change_pos(1.0, 1.0, 1.0);
    } 

    image = flip_vertical(&image);
    image.save("image.png").unwrap();

    // follow the rays, determining the 3 best
    
    // calculate the strength of the 3 best rays
     
}
