use sdl2::rect::Rect;

use crate::maze_generation::{Maze, Node};

/// The size of the borders, assuming the squares are one unit
const BORDER_SIZE: f64 = 0.25;

/// Generates the rects that need to be drawn for the maze
pub fn maze_rects(maze: &Maze, draw_size: (u32, u32)) -> Vec<Rect> {
    // Computer the size of the maze in maze units
    let visual_maze_width = f64::from(maze.width as u32).mul_add(1.0 + BORDER_SIZE, BORDER_SIZE);
    let visual_maze_height = f64::from(maze.height as u32).mul_add(1.0 + BORDER_SIZE, BORDER_SIZE);

    // Compute the size of the rects I need to draw
    let block_horizontal = f64::from(draw_size.0) / visual_maze_width;
    let block_vertical = f64::from(draw_size.1) / visual_maze_height;
    let block_length: i32 = Ord::min(block_horizontal as _, block_vertical as _);
    let border_length: i32 = (f64::from(block_length) * BORDER_SIZE) as _;
    let block_and_border_length = block_length + border_length;

    // Computer the necessary shifts to center the maze in top, left format
    let full_maze_width = (visual_maze_width * f64::from(block_length)) as i32;
    let horizontal_shift = (draw_size.0 as i32 - full_maze_width) / 2 + border_length;
    let full_maze_height = (visual_maze_height * f64::from(block_length)) as i32;
    let vertical_shift = (draw_size.1 as i32 - full_maze_height) / 2 + border_length;

    // The list of rects to draw at the end
    let mut rects_to_draw: Vec<Rect> = Vec::new();

    for (x, column) in maze.nodes.iter().enumerate() {
        for (y, &node) in column.iter().enumerate() {
            let xcoord: i32 = horizontal_shift + (x as i32 * block_and_border_length);
            let ycoord: i32 = vertical_shift + (y as i32 * block_and_border_length);
            rects_to_draw.append(&mut node_to_rects(
                node,
                block_length as u32,
                xcoord,
                ycoord,
            ));
        }
    }

    rects_to_draw
}

/// Generates the rects that need to be drawn for a node. Generates the main
/// node at origin relative to shifts
fn node_to_rects(node: Node, block_length: u32, hshift: i32, vshift: i32) -> Vec<Rect> {
    if !node.contains(Node::marked) {
        return Vec::new();
    }

    let mut rects = vec![Rect::new(hshift, vshift, block_length, block_length)];

    rects.extend((node & Node::directions).iter().map(|dir| match dir {
        Node::north => Rect::new(
            hshift,
            vshift - block_length as i32 / 2,
            block_length,
            block_length / 2,
        ),
        Node::east => Rect::new(
            hshift + block_length as i32,
            vshift,
            block_length / 2,
            block_length,
        ),
        Node::south => Rect::new(
            hshift,
            vshift + block_length as i32,
            block_length,
            block_length / 2,
        ),
        Node::west => Rect::new(
            hshift - block_length as i32 / 2,
            vshift,
            block_length / 2,
            block_length,
        ),
        _ => panic!(),
    }));

    rects
}
