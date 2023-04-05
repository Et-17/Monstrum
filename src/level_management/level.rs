use crate::maze_generation::Maze;

/// This struct contains everything there is too know about the current level.
pub struct Level {
    pub maze: Maze,
    /// The color of the maze walls
    pub foreground: (u8, u8, u8),
    /// The color of the open maze cells
    pub background: (u8, u8, u8),
}

impl Level {
    /// Creates a new Level struct
    pub fn new(maze: Maze, foreground: (u8, u8, u8), background: (u8, u8, u8)) -> Level {
        Level {
            maze,
            foreground,
            background,
        }
    }
}
