extern crate sdl2;

mod controls;
mod graphics;
mod random;

use controls::InputHandler;
use graphics::GraphicsHandler;

fn main() -> Result<(), String> {
    let mut context = sdl2::init()?;
    let mut graphics =
        graphics::GraphicsHandler::initialize(String::from("Test Window"), &mut context)?;
    let mut input = controls::InputHandler::initialize(&mut context)?;

    loop {
        step(&mut graphics, &mut input);
    }
}

fn step(graphics: &mut GraphicsHandler, input: &mut InputHandler) {
    input.handle_events();
    graphics.do_frame();
}
