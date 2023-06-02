mod block;
mod constants;
mod game;
mod orb;
mod player;
mod spike;
mod surface_result;
mod util;
mod vector;

use crate::constants::{HEIGHT, WIDTH};
use crate::game::Game;
use constants::{FPS, GROUND_Y_FLIP, GROUND_Y_NORMAL, SELECTED_LEVEL};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::{Button, ButtonEvent, ButtonState, EventLoop, Key, MouseButton, WindowSettings};
use std::time::{Duration, Instant};

fn main() {
    let opengl: OpenGL = OpenGL::V4_5;

    let mut window: Window = WindowSettings::new("Geometry Dash", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .resizable(false)
        .exit_on_esc(false)
        .vsync(true)
        .samples(4)
        .build()
        .unwrap();

    let mut game: Game = Game::new();
    let mut fps: i32 = 0;
    let mut fps_counter: i32 = 0;
    let mut last_update: Instant = Instant::now();

    game.initialize_level(SELECTED_LEVEL);

    let mut events: Events = Events::new(EventSettings::new()).max_fps(FPS);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            let now: Instant = Instant::now();
            let elapsed: Duration = now - last_update;
            fps_counter += 1;

            if elapsed.as_secs_f32() >= 1.0 {
                fps = fps_counter;
                fps_counter = 0;
                last_update = now;
            }

            game.render(&args, fps);
        }

        if let Some(args) = e.update_args() {
            if !game.frozen {
                game.update(&args);
            }
        }

        if let Some(args) = e.button_args() {
            let state: ButtonState = args.state;
            let button: Button = args.button;

            match button {
                Button::Mouse(mouse_button) => match mouse_button {
                    MouseButton::Left => {
                        if !game.frozen {
                            game.player.jumping = state == ButtonState::Press;
                        }
                    }
                    MouseButton::Right => {
                        if state == ButtonState::Press && !game.frozen && !game.player.crashed {
                            game.player.gravity_flip = !game.player.gravity_flip;
                            game.player.acc.y = -game.player.acc.y;
                            game.player.grounded = false;
                            game.player.jump.y = if game.player.gravity_flip {
                                GROUND_Y_FLIP
                            } else {
                                GROUND_Y_NORMAL
                            }
                        }
                    }
                    _ => {}
                },
                Button::Keyboard(keyboard_button) => match keyboard_button {
                    Key::Escape => {
                        if state == ButtonState::Press && !game.player.crashed && !game.victory {
                            game.frozen = !game.frozen;
                        }
                    }
                    Key::R => {
                        if state == ButtonState::Press
                            && (game.frozen || game.player.crashed || game.victory)
                        {
                            game.initialize_level(SELECTED_LEVEL);
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
