#[derive(Debug)]
pub struct Sphere {
    pub radius: f64,
    pub height: f64,
}

impl Sphere {
    pub fn area(&self) -> f64 {
        f64::powi(self.radius, 2) * self.height * std::f64::consts::PI
    }
    pub fn can_hold(&self, other: &Sphere) -> bool {
        self.height > other.height && self.radius > other.radius
    }
}
