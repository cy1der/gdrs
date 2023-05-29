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
    pub fn new(x: f32, y: f32, size_x: f32, size_y: f32) -> Self {
        Block {
            pos: Vector::new(x, y),
            size: Vector::new(size_x, size_y),
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
