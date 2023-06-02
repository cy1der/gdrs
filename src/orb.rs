use crate::vector::Vector;

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
}
