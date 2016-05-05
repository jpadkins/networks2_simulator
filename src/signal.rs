use std::f64;

pub struct Signal;

#[allow(dead_code)]
impl Signal {
    pub fn friis(pt: &f64, gt: &f64, gr: &f64, lamda: &f64, d: &f64) -> f64 {
        pt*gt*gr * ((lamda / (4.0*f64::consts::PI*d)).powi(2))
    }
    pub fn aplitude(frequency: f64, distance: f64) -> f64 {
        distance / frequency
    }
}
