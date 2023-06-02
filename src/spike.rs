use crate::vector::Vector;

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
}
