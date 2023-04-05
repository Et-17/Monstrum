//! This manages the graphics system. First you need to run
//! GraphicsHandler::initialize, which will create the window, then call
//! do_frame() in order to draw the next frame.

use crate::{
    level_management::Level,
    maze_generation::{Maze, Node},
};

/// This is the primary struct that gets passed around. It can contain whatever
/// is necessary for the graphics system being used. Because this module uses SDL2,
/// it contains the canvas.
pub struct GraphicsHandler {
    pub canvas: sdl2::render::WindowCanvas,
}

impl GraphicsHandler {
    /// Creates a new GraphicsHandler struct
    fn new(canvas: sdl2::render::WindowCanvas) -> GraphicsHandler {
        GraphicsHandler { canvas }
    }

    /// Creates a new fullscreen_desktop window with the title given
    pub fn initialize(
        title: String,
        sdl_context: &mut sdl2::Sdl,
    ) -> Result<GraphicsHandler, String> {
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(title.as_str(), 0, 0)
            .fullscreen_desktop()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(GraphicsHandler::new(canvas))
    }

    /// Draws the next frame
    pub fn do_frame(&mut self) {
        self.canvas.present();
    }

    /// Draw a maze. Assumes that foreground is the color for open cells
    pub fn draw_maze(&mut self, maze: &Maze) -> Result<(), String> {
        // Compute the size of the rects I need to draw
        let screen_size = self.canvas.output_size()?;
        let block_horizontal = screen_size.0 / (maze.width * 2 + 1) as u32;
        let block_vertical = screen_size.1 / (maze.height * 2 + 1) as u32;

        // This returns a maze that is flattened and labeled with coords
        let labeled_maze = maze.nodes.iter().enumerate().flat_map(|(x, column)| {
            column
                .iter()
                .enumerate()
                .map(move |(y, &node)| (x, y, node))
        });

        // This returns a maze that is filtered to only have the marked nodes
        let marked_labeled_maze = labeled_maze.filter(|(_, _, node)| node.contains(Node::marked));

        for (x, y, node) in marked_labeled_maze {
            let cell_coord = (x as i32 * 2 + 1, y as i32 * 2 + 1);

            let coords_to_draw = node.iter().map(|flag| match flag {
                Node::marked => cell_coord,
                Node::north => (cell_coord.0, cell_coord.1 - 1),
                Node::south => (cell_coord.0, cell_coord.1 + 1),
                Node::east => (cell_coord.0 + 1, cell_coord.1),
                Node::west => (cell_coord.0 - 1, cell_coord.1),
                _ => (0, 0), // There aren't anymore flags, but the comp doesn't know that
            });

            let rects_to_draw = coords_to_draw.map(|(rect_x, rect_y)| {
                sdl2::rect::Rect::new(
                    rect_x * block_horizontal as i32,
                    rect_y * block_vertical as i32,
                    block_horizontal,
                    block_vertical,
                )
            });

            self.canvas
                .fill_rects(&rects_to_draw.collect::<Vec<sdl2::rect::Rect>>())?;
        }

        Ok(())
    }

    /// Draw a level
    pub fn draw_level(&mut self, level: &Level) -> Result<(), String> {
        self.canvas.set_draw_color(level.background);
        self.canvas.clear();
        self.canvas.set_draw_color(level.foreground);
        self.draw_maze(&level.maze)?;

        Ok(())
    }
}
