use crate::constants::{HEIGHT, WIDTH};
use crate::vector::Vector;

pub struct Spike {
    pub pos: Vector,
    pub size: Vector,
    pub flip: bool,
    pub height: f64,
    pub vertices: [[f64; 2]; 3],
}

impl Spike {
    pub fn new(x: f32, y: f32, size_x: f32, size_y: f32, flip: bool) -> Self {
        Spike {
            pos: Vector::new(x, y),
            size: Vector::new(size_x, size_y),
            flip,
            height: if flip {
                (y + size_y) as f64
            } else {
                (y - size_y) as f64
            },
            vertices: [
                [(x - (size_x / 2.0)) as f64, y as f64],
                [(x + (size_x / 2.0)) as f64, y as f64],
                [
                    x as f64,
                    if flip {
                        (y - size_y) as f64
                    } else {
                        (y + size_y) as f64
                    },
                ],
            ],
        }
    }

    pub fn is_on_screen(&self) -> bool {
        WIDTH as f32 >= self.pos.x
            && 0.0 <= self.pos.x + self.size.x
            && HEIGHT as f32 >= self.pos.y
            && 0.0 <= self.pos.y + self.size.y
    }
}
