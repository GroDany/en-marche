use crate::point::EMPoint;

pub trait Distance {
    fn distance(&self, p: &EMPoint) -> f64;
}

pub struct Sphere {
    pub center: EMPoint,
    pub radius: f64,
}

impl Distance for Sphere {
    fn distance(&self, p: &EMPoint) -> f64 {
        return p.sub_point(&self.center).magnitude() - self.radius;
    }
}
