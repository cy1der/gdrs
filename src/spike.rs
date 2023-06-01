use crate::{
    constants::{HEIGHT, WIDTH},
    vector::Vector,
};

pub struct Spike {
    pub pos: Vector,
    pub size: Vector,
    pub flip: bool,
    pub height: f64,
    pub vertices: [[f64; 2]; 3],
}

impl Spike {
    pub fn new(pos: Vector, size: Vector, flip: bool) -> Self {
        Spike {
            flip,
            height: if flip {
                (pos.y + size.y) as f64
            } else {
                (pos.y - size.y) as f64
            },
            vertices: [
                [(pos.x - (size.x / 2.0)) as f64, pos.y as f64],
                [(pos.x + (size.x / 2.0)) as f64, pos.y as f64],
                [
                    pos.x as f64,
                    if flip {
                        (pos.y - size.y) as f64
                    } else {
                        (pos.y + size.y) as f64
                    },
                ],
            ],
            pos,
            size,
        }
    }

    pub fn is_on_screen(&self) -> bool {
        WIDTH as f32 >= self.pos.x
            && 0.0 <= self.pos.x + self.size.x
            && HEIGHT as f32 >= self.pos.y
            && 0.0 <= self.pos.y + self.size.y
    }
}
