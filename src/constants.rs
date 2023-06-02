pub const WIDTH: u32 = 1920;
pub const HEIGHT: u32 = 1080;

pub const BG_COLOR: [f32; 4] = [0.25, 0.25, 0.25, 1.0];
pub const PLAYER_COLOR: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
pub const GROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const GROUND_COLOR_TRANSPARENT: [f32; 4] = [0.0, 0.0, 0.0, 0.5];
pub const BLOCK_COLOR: [f32; 4] = GROUND_COLOR;
pub const SPIKE_COLOR: [f32; 4] = GROUND_COLOR;
pub const ORB_COLOR: [f32; 4] = [1.0, 1.0, 0.0, 1.0];

pub const GROUND_Y_NORMAL: f32 = HEIGHT as f32 - (HEIGHT as f32 * 0.15);
pub const GROUND_Y_FLIP: f32 = HEIGHT as f32 * 0.15;

pub const PLAYER_SIZE: u32 = 50;
pub const PLAYER_SPEED: f32 = 10.386 * 60.0;

pub const GRAVITY: f32 = 0.575 * 60.0;

pub const FPS: u64 = 60;

pub const SELECTED_LEVEL: &str = "level_1"; // Change this based on file names in levels/
