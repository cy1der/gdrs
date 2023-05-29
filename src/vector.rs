#[derive(PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Vector { x, y }
    }

    pub fn dist(&self, other: &Vector) -> f32 {
        let dx: f32 = self.x - other.x;
        let dy: f32 = self.y - other.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}
