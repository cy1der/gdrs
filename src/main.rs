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
use constants::{GROUND_Y_FLIP, GROUND_Y_NORMAL, SELECTED_LEVEL};
use glutin_window::GlutinWindow;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::{Button, ButtonEvent, ButtonState, Key, MouseButton, WindowSettings};

fn main() {
    let opengl: OpenGL = OpenGL::V4_5;

    let mut window: GlutinWindow = WindowSettings::new("Geometry Dash", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .resizable(false)
        .exit_on_esc(false)
        .build()
        .unwrap();

    let mut game: Game = Game::new();

    game.initialize_level(SELECTED_LEVEL);

    let mut events: Events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
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
