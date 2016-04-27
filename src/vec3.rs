use std::fmt;

#[allow(dead_code)]
#[derive(PartialEq, PartialOrd, Clone)] 
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

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
    pub fn pos(&mut self, x: f64, y: f64, z: f64) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
