use crate::{
    constants::{HEIGHT, WIDTH},
    vector::Vector,
};

#[derive(PartialEq)]
pub struct Block {
    pub pos: Vector,
    pub size: Vector,
    pub on_screen: bool,
}

impl Block {
    pub fn new(pos: Vector, size: Vector) -> Self {
        Block {
            pos,
            size,
            on_screen: false,
        }
    }

    pub fn is_on_screen(&self) -> bool {
        WIDTH as f32 >= self.pos.x
            && 0.0 <= self.pos.x + self.size.x
            && HEIGHT as f32 >= self.pos.y
            && 0.0 <= self.pos.y + self.size.y
    }
}
