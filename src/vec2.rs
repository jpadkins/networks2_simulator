use std::fmt;
use std::mem;

#[derive(PartialEq, PartialOrd, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[allow(dead_code)]
impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 {
            x: x,
            y: y,
        }
    }
    pub fn line(v1: &Vec2, v2: &Vec2) -> Vec<Vec2> {
        let mut line = Vec::new();
        let mut src = Vec2::new(v1.x, v1.y);
        let mut dst = Vec2::new(v2.x, v2.y);
        let steep = (dst.y - src.y).abs() > (dst.x - src.x).abs();
        if steep {
            mem::swap(&mut src.x, &mut src.y);
            mem::swap(&mut dst.x, &mut dst.y);
        }
        if src.x > dst.x {
            mem::swap(&mut src.x, &mut dst.x);
            mem::swap(&mut src.y, &mut dst.y);
        }
        let dx = dst.x - src.x;
        let dy = (dst.y - src.y).abs();
        let mut err = dx / 2.0;
        let mut ystep = 0;
        if src.y < dst.y {
            ystep = 1;
        } else {
            ystep = -1;
        }
        let mut y = src.y as i32;
        let max_x = dst.x as i32;
        let mut x = src.x as i32; 
        while x < max_x {
            let color = [((255 / max_x)*x) as u8, 0, 0];
            match steep {
                true    => { line.push(Vec2::new(y as f64, x as f64)); },
                false   => { line.push(Vec2::new(x as f64, y as f64)); },
            }
            err -= dy;
            if err < 0.0 {
                y += ystep;
                err += dx;
            }
            x += 1;
        }
        line
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
