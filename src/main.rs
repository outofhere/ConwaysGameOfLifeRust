extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::EventLoop;
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};

pub use crate::gameboard::Gameboard;
pub use crate::gameboard_controller::GameboardController;
pub use crate::gameboard_view::{GameboardView, GameboardViewSettings};

mod gameboard;
mod gameboard_controller;
mod gameboard_view;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Conway's 'Game of Life' beta", [400; 2])
        .graphics_api(opengl)
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let gameboard = Gameboard::new();
    let mut gameboard_controller = GameboardController::new(gameboard);
    let gameboard_view_settigns = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settigns);

    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(&e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};

                clear([1.0; 4], g);
                gameboard_view.draw(&gameboard_controller, &c, g);
            });
        }
    }
}
