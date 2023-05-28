use crate::constants::{GRAVITY, GROUND_Y_NORMAL, PLAYER_SIZE, PLAYER_SPEED, WIDTH};
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
                GROUND_Y_NORMAL as f32 - (PLAYER_SIZE as f32 / 2.0),
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
}
