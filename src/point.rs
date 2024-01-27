pub struct EMPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl EMPoint {
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn unit(&self) -> Self {
        self.mult_scalar(1.0 / self.norm())
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

    pub fn add_x(&self, n: f64) -> Self {
        Self {
            x: self.x + n,
            y: self.y,
            z: self.z,
        }
    }

    pub fn add_y(&self, n: f64) -> Self {
        Self {
            x: self.x,
            y: self.y + n,
            z: self.z,
        }
    }

    pub fn add_z(&self, n: f64) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z + n,
        }
    }

    pub fn sub_x(&self, n: f64) -> Self {
        Self {
            x: self.x - n,
            y: self.y,
            z: self.z,
        }
    }

    pub fn sub_y(&self, n: f64) -> Self {
        Self {
            x: self.x,
            y: self.y - n,
            z: self.z,
        }
    }

    pub fn sub_z(&self, n: f64) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z - n,
        }
    }

    pub fn dot(&self, point: &EMPoint) -> f64 {
        self.x * point.x + self.y * point.y + self.z * point.z
    }
}
