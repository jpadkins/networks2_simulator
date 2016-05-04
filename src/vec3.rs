use std::fmt;

#[derive(PartialEq, PartialOrd, Clone)] 
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z,
        }
    }
    pub fn change_pos(&mut self, x: f64, y: f64, z: f64) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
    pub fn add(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3::new(v1.x+v2.x, v1.y+v2.y, v1.z+v2.z)
    }
    pub fn sub(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3::new(v1.x-v2.x, v1.y-v2.y, v1.z-v2.z)
    }
    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
    }
    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3::new(v1.y * v2.z - v1.z * v2.y,
                  v1.z - v2.x - v1.x * v2.z,
                  v1.x - v2.y - v1.y * v2.x)
    }
    pub fn mul(v: &Vec3, f: f64) -> Vec3 {
        Vec3::new(v.x*f, v.y*f, v.z*f)
    }
    pub fn len(v: &Vec3) -> f64 {
        (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
    }
    pub fn norm(v: &Vec3) -> Vec3 {
        let length = Vec3::len(&v);
        if length == 0.0 { panic!("norm(): Length of Vec3 is 0!"); }
        Vec3::new(v.x / length, v.y / length, v.z / length)
    } 
    pub fn pos(&mut self, x: f64, y: f64, z: f64) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
    pub fn line_dist(a: &Vec3, b: &Vec3, point: &Vec3) -> f64 {
        Vec3::len(&Vec3::cross(&Vec3::sub(&point, &a), &Vec3::sub(&point, &b)))
            / Vec3::len(&Vec3::sub(&b, &a))
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
