use crate::vector::Vector;

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
}
