use crate::block::Block;
use crate::constants::{GRAVITY, GROUND_Y_FLIP, GROUND_Y_NORMAL, PLAYER_SIZE, PLAYER_SPEED, WIDTH};
use crate::spike::Spike;
use crate::surface_result::SurfaceResult;
use crate::util::*;
use crate::vector::Vector;

pub struct Player {
    pub size: u32,
    pub angle: f32,
    pub grounded: bool,
    pub gravity_flip: bool,
    pub jumping: bool,
    pub crashed: bool,
    pub pos: Vector,
    pub vel: Vector,
    pub acc: Vector,
    pub jump: Vector,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            size: PLAYER_SIZE,
            angle: 0.0,
            grounded: true,
            gravity_flip: false,
            jumping: false,
            crashed: false,
            pos: Vector::new(
                WIDTH as f32 * 0.2,
                GROUND_Y_NORMAL - (PLAYER_SIZE as f32 / 2.0),
            ),
            vel: Vector::new(PLAYER_SPEED, 0.0),
            acc: Vector::new(0.0, GRAVITY),
            jump: Vector::new(f32::MAX, f32::MAX),
        }
    }
}

impl Player {
    pub fn new() -> Self {
        Player {
            ..Default::default()
        }
    }

    pub fn on_ground(&self) -> SurfaceResult {
        let mut result: bool = false;
        let mut y: f32 = 0.0;

        if self.pos.y + (self.size / 2) as f32 >= GROUND_Y_NORMAL && !self.gravity_flip {
            result = true;
            y = GROUND_Y_NORMAL - (self.size / 2) as f32;
        } else if self.pos.y - (self.size / 2) as f32 <= GROUND_Y_FLIP && self.gravity_flip {
            result = true;
            y = GROUND_Y_FLIP + (self.size / 2) as f32;
        }

        if result {
            SurfaceResult::OnSurface(y)
        } else {
            SurfaceResult::NotOnSurface
        }
    }

    pub fn on_block(&self, block: &Block) -> SurfaceResult {
        let mut result: bool = false;
        let mut y: f32 = 0.0;

        if self.pos.x + (self.size as f32 / 2.0) > block.pos.x
            && self.pos.x - (self.size as f32 / 2.0) < block.pos.x + block.size.x
        {
            if self.gravity_flip {
                if self.pos.y - (self.size as f32 / 2.0) < block.pos.y + block.size.y
                    && self.pos.y + (self.size as f32 / 2.0) > block.pos.y + block.size.y
                {
                    result = true;
                    y = block.pos.y + block.size.y + (self.size as f32 / 2.0);
                }
            } else if self.pos.y - (self.size as f32 / 2.0) < block.pos.y
                && self.pos.y + (self.size as f32 / 2.0) > block.pos.y
            {
                result = true;
                y = block.pos.y - (self.size as f32 / 2.0);
            }
        }

        if result {
            SurfaceResult::OnSurface(y)
        } else {
            SurfaceResult::NotOnSurface
        }
    }

    pub fn check_block_crash(&mut self, block: &Block) {
        self.crashed = self.pos.x + (self.size as f32 / 2.0) >= block.pos.x
            && self.pos.x - (self.size as f32 / 2.0) <= block.pos.x + block.size.x
            && ((self.gravity_flip
                && self.pos.y + (self.size as f32 / 2.0) >= block.pos.y
                && self.pos.y - (self.size as f32 / 2.0) <= block.pos.y)
                || (!self.gravity_flip
                    && self.pos.y - (self.size as f32 / 2.0) <= block.pos.y + block.size.y
                    && self.pos.y + (self.size as f32 / 2.0) >= block.pos.y + block.size.y));
    }

    pub fn check_spike_crash(&mut self, spike: &Spike) {
        let mut n: usize;
        let mut i: usize = 0;

        while i < spike.vertices.len() {
            n = i + 1;
            if n == spike.vertices.len() {
                n = 0;
            }

            let vc: [f64; 2] = spike.vertices[i];
            let vn: [f64; 2] = spike.vertices[n];

            let collision: bool = line_rect(
                &Vector::new(vc[0] as f32, vc[1] as f32),
                &Vector::new(vn[0] as f32, vn[1] as f32),
                &Vector::new(
                    self.pos.x - (self.size as f32 / 2.0),
                    self.pos.y - (self.size as f32 / 2.0),
                ),
                &Vector::new(self.size as f32, self.size as f32),
            );
            let inside: bool = polygon_point(
                spike.vertices,
                Vector::new(
                    self.pos.x - (self.size as f32 / 2.0),
                    self.pos.y - (self.size as f32 / 2.0),
                ),
            );

            self.crashed = collision || inside;

            if self.crashed {
                break;
            } else {
                i += 1;
            }
        }
    }
}
