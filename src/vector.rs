pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Vector {
        Vector { x, y }
    }

    pub fn add(&mut self, v: &Vector) {
        self.x += v.x;
        self.y += v.y;
    }

    pub fn sub(&mut self, v: &Vector) {
        self.x -= v.x;
        self.y -= v.y;
    }

    pub fn mult(&mut self, n: f32) {
        self.x *= n;
        self.y *= n;
    }

    pub fn div(&mut self, n: f32) {
        self.x /= n;
        self.y /= n;
    }

    pub fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&mut self) {
        let m = self.mag();
        if m != 0.0 {
            self.div(m);
        }
    }

    pub fn limit(&mut self, max: f32) {
        let m = self.mag();
        if m > max {
            self.normalize();
            self.mult(max);
        }
    }
}
