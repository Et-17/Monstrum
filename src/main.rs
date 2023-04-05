extern crate sdl2;

mod controls;
mod graphics;
mod level_management;
mod maze_generation;
mod random;

use controls::InputHandler;
use graphics::GraphicsHandler;
use level_management::Level;
use maze_generation::{Maze, Node};

fn main() -> Result<(), String> {
    let mut context = sdl2::init()?;
    let mut graphics =
        graphics::GraphicsHandler::initialize(String::from("Test Window"), &mut context)?;
    let mut input = controls::InputHandler::initialize(&mut context)?;

    let maze = Maze::new(&vec![
        vec![
            Node::marked | Node::south | Node::east,
            Node::marked | Node::north,
        ],
        vec![Node::marked | Node::west, Node::empty()],
    ]);
    let level = Level::new(maze, (255, 255, 255), (0, 0, 0));

    graphics.draw_level(&level)?;

    loop {
        step(&mut graphics, &mut input);
    }
}

fn step(graphics: &mut GraphicsHandler, input: &mut InputHandler) {
    input.handle_events();
    graphics.do_frame();
}
