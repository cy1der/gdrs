use crate::constants::BG_COLOR;
use crate::player::Player;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};

pub struct Game {
    pub gl: GlGraphics,
    pub frozen: bool,
    pub attempt_count: u32,
    pub victory: bool,
    pub player: Player,
}

impl Default for Game {
    fn default() -> Self {
        let opengl: OpenGL = OpenGL::V4_5;

        Game {
            gl: GlGraphics::new(opengl),
            player: Player::new(),
            frozen: false,
            attempt_count: 1,
            victory: false,
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

        self.gl
            .draw(args.viewport(), |c: Context, gl: &mut GlGraphics| {
                clear(BG_COLOR, gl);
            });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // args.dt
    }
}
