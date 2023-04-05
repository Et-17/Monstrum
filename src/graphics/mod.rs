//! This module manages the graphics of the game. All the other code won't
//! directly interact with the user, aside from console output, and it will all
//! go through this module. Doing this allows me to easily port to new systems
//! that need different ways of interacting with the user. Currently it is using
//! SDL2 which will allow it to work on Linux, MacOS, and Windows, but you never
//! know. Also it just keeps everything nice and organized.

mod handler;
pub use handler::*;
