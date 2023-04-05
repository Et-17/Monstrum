//! This module manages the generation of the level mazes. Not really that much
//! more to it. It just does the actual structure of the maze. Placement of
//! things like guns and monsters are handled in the level_management module.

mod maze;
pub use maze::*;
mod node;
pub use node::*;
