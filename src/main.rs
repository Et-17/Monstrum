extern crate sdl2;

mod controls;
mod graphics;
mod level_management;
mod maze_generation;
mod random;

use controls::InputHandler;
use graphics::GraphicsHandler;
use level_management::Level;
use maze_generation::Maze;
use random::Generator;

fn main() -> Result<(), String> {
    let mut context = sdl2::init()?;
    let mut graphics =
        graphics::GraphicsHandler::initialize(String::from("Test Window"), &mut context)?;
    let mut input = controls::InputHandler::initialize(&mut context)?;

    let mut generator = Generator::new(40, 20);

    println!("Generating maze");
    let maze = Maze::backtrack(10, 10, &mut generator.birth());
    println!("Finished generating maze");
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
