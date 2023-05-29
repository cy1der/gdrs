use crate::block::Block;
use crate::constants::{
    BG_COLOR, BLOCK_COLOR, GROUND_COLOR, GROUND_Y_FLIP, GROUND_Y_NORMAL, HEIGHT, PLAYER_COLOR,
    SPIKE_COLOR, WIDTH,
};
use crate::player::Player;
use crate::spike::Spike;
use crate::surface_result::SurfaceResult;
use crate::vector::Vector;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};

pub struct Game {
    pub gl: GlGraphics,
    pub frozen: bool,
    pub attempt_count: u32,
    pub victory: bool,
    pub player: Player,
    pub blocks: Vec<Block>,
    pub spikes: Vec<Spike>,
}

impl Default for Game {
    fn default() -> Self {
        let opengl: OpenGL = OpenGL::V4_5;

        Game {
            gl: GlGraphics::new(opengl),
            player: Player::new(),
            frozen: true,
            attempt_count: 1,
            victory: false,
            blocks: Vec::new(),
            spikes: Vec::new(),
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            ..Default::default()
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let player_square: [f64; 4] = rectangle::square(0.0, 0.0, self.player.size as f64);
        let ground_rect: [f64; 4] = rectangle::rectangle_by_corners(
            0.0,
            if self.player.gravity_flip {
                0.0
            } else {
                GROUND_Y_NORMAL as f64
            },
            WIDTH as f64,
            if self.player.gravity_flip {
                GROUND_Y_FLIP as f64
            } else {
                HEIGHT as f64
            },
        );

        self.gl
            .draw(args.viewport(), |c: Context, gl: &mut GlGraphics| {
                clear(BG_COLOR, gl);

                let player_transform: [[f64; 3]; 2] = c
                    .transform
                    .trans(self.player.pos.x as f64, self.player.pos.y as f64)
                    .rot_deg(if self.player.pos.x > self.player.jump.x {
                        -self.player.angle as f64
                    } else {
                        self.player.angle as f64
                    })
                    .trans(
                        -(self.player.size as f64) / 2.0,
                        -(self.player.size as f64) / 2.0,
                    );

                rectangle(PLAYER_COLOR, player_square, player_transform, gl);

                let ground_transform: [[f64; 3]; 2] = c.transform.trans(0.0, 0.0);

                rectangle(GROUND_COLOR, ground_rect, ground_transform, gl);

                for block in self.blocks.iter() {
                    if block.is_on_screen() {
                        let block_rect: [f64; 4] = rectangle::rectangle_by_corners(
                            block.pos.x as f64,
                            block.pos.y as f64,
                            (block.pos.x + block.size.x) as f64,
                            (block.pos.y + block.size.y) as f64,
                        );

                        rectangle(BLOCK_COLOR, block_rect, c.transform, gl);
                    }
                }

                for spike in self.spikes.iter() {
                    if spike.is_on_screen() {
                        polygon(SPIKE_COLOR, &spike.vertices, c.transform, gl);
                    }
                }
            });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        println!("{}", self.victory);
        if !self.player.crashed && !self.frozen {
            self.player.vel.y += self.player.acc.y;
            self.player.pos.y += self.player.vel.y * args.dt as f32;
            self.player.jump.x -= self.player.vel.x * args.dt as f32;

            if !self.player.grounded {
                let d: f32 = self.player.pos.dist(&self.player.jump);

                let angle: f32 = if self.player.gravity_flip {
                    -((self.player.pos.y - self.player.jump.y) / d)
                        .asin()
                        .to_degrees()
                } else {
                    ((self.player.jump.y - self.player.pos.y) / d)
                        .asin()
                        .to_degrees()
                };

                self.player.angle = angle;
            }

            let ground_check: SurfaceResult = self.player.on_ground();

            match ground_check {
                SurfaceResult::OnSurface(y) => {
                    self.player.angle = 0.0;
                    self.player.grounded = true;
                    self.player.jump = Vector::new(f32::MAX, f32::MAX);
                    self.player.pos.y = y;
                    self.player.vel.y = 0.0;
                }
                SurfaceResult::NotOnSurface => {}
            }

            let mut no_blocks: bool = self.blocks.is_empty();
            let mut no_spikes: bool = self.spikes.is_empty();

            for i in 0..self.blocks.len() {
                let block = &mut self.blocks[i];
                block.pos.x -= self.player.vel.x * args.dt as f32;

                if block.pos.x < 0.0 && !block.is_on_screen() {
                    self.blocks.remove(i);

                    if self.blocks.is_empty() {
                        no_blocks = true;
                        break;
                    } else {
                        continue;
                    }
                }

                if block.is_on_screen() {
                    let surface_check: SurfaceResult = self.player.on_block(block);

                    match surface_check {
                        SurfaceResult::OnSurface(y) => {
                            self.player.angle = 0.0;
                            self.player.grounded = true;
                            self.player.jump = Vector::new(f32::MAX, f32::MAX);
                            self.player.vel.y = 0.0;
                            self.player.pos.y = y;
                        }
                        SurfaceResult::NotOnSurface => {}
                    }

                    self.player.check_block_crash(block);
                }
            }

            for i in 0..self.spikes.len() {
                let spike = &mut self.spikes[i];
                spike.pos.x -= self.player.vel.x * args.dt as f32;

                if spike.pos.x < 0.0 && !spike.is_on_screen() {
                    self.spikes.remove(i);

                    if self.spikes.is_empty() {
                        no_spikes = true;
                        break;
                    } else {
                        continue;
                    }
                }

                if spike.is_on_screen() {
                    self.player.check_spike_crash(spike);
                }
            }

            self.victory = no_blocks && no_spikes;

            if self.player.grounded && self.player.jumping {
                self.player.jump = Vector::new(self.player.pos.x + 250.0, self.player.pos.y);
                self.player.grounded = false;
                self.player.vel.y = if self.player.gravity_flip {
                    self.player.acc.y.powi(2)
                } else {
                    -self.player.acc.y.powi(2)
                };
            }
        }
    }
}
