//! This module manages the controls of the game. Every OS manages controls a
//! little differently, so all platform dependent code is organized into here in
//! the same way as the graphics module to allow easy porting to different
//! systems. I am using SDL2 to handle the input in this implementation,
//! however, which will cover Linux, MacOS, and Windows.

mod handler;
pub use handler::*;
