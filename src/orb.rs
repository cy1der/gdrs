use crate::{
    constants::{HEIGHT, WIDTH},
    vector::Vector,
};

pub struct Orb {
    pub pos: Vector,
    pub d: f32,
    pub activated: bool,
}

impl Orb {
    pub fn new(pos: Vector, d: f32) -> Self {
        Orb {
            pos,
            d,
            activated: false,
        }
    }

    pub fn is_on_screen(&self) -> bool {
        self.pos.x + (self.d / 2.0) <= WIDTH as f32
            && self.pos.x - (self.d / 2.0) >= 0.0
            && self.pos.y + (self.d / 2.0) <= HEIGHT as f32
            && self.pos.y - (self.d / 2.0) >= 0.0
    }
}
