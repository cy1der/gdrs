mod constants;
mod game;
mod player;
mod vector;

use crate::constants::{FPS, HEIGHT, WIDTH};
use crate::game::Game;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::{Button, ButtonEvent, ButtonState, EventLoop, MouseButton, TextEvent, WindowSettings};

fn main() {
    let opengl: OpenGL = OpenGL::V4_5;

    let mut window: Window = WindowSettings::new("Geometry Dash", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(false)
        .build()
        .unwrap();

    let mut game: Game = Game::new();
    let mut events: Events = Events::new(EventSettings::new().max_fps(FPS));

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

            if button == Button::Mouse(MouseButton::Left) {
                // left click (activates on both press and release)
            }
        }

        if let Some(letter) = e.text_args() {
            // Keyboard text
        }
    }
}
