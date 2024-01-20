pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn norm(&self) -> Self {
        self.mult_scalar(1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt())
    }

    pub fn add_point(&self, n: &Point) -> Self {
        Self {
            x: self.x + n.x,
            y: self.y + n.y,
            z: self.z + n.z,
        }
    }

    pub fn add_scalar(&self, n: f64) -> Self {
        Self {
            x: self.x + n,
            y: self.y + n,
            z: self.z + n,
        }
    }

    pub fn sub_point(&self, n: &Point) -> Self {
        Self {
            x: self.x - n.x,
            y: self.y - n.y,
            z: self.z - n.z,
        }
    }

    pub fn sub_scalar(&self, n: f64) -> Self {
        Self {
            x: self.x - n,
            y: self.y - n,
            z: self.z - n,
        }
    }

    pub fn mult_scalar(&self, n: f64) -> Self {
        Self {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
        }
    }
}

pub trait Distance {
    fn distance(&self, p: Point) -> f64;
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Distance for Sphere {
    fn distance(&self, p: Point) -> f64 {
        return p.sub_point(&self.center).length() - self.radius;
    }
}
