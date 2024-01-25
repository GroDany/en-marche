pub struct EMPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl EMPoint {
    pub fn from(point: &EMPoint) -> Self {
        Self {
            x: point.x,
            y: point.y,
            z: point.z,
        }
    }
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn norm(&self) -> Self {
        self.mult_scalar(1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt())
    }

    pub fn add_point(&self, n: &EMPoint) -> Self {
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

    pub fn sub_point(&self, n: &EMPoint) -> Self {
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
