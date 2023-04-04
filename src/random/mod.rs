//! A module for generating random numbers used for everything from level
//! generation to mob AI. Yes, I know that the crate rand works and is fine and
//! everything, but I just want to be sure that nothing will change across
//! versions. Also I use it so much I want to be able to completely verify that
//! it is a good algorithm with good speed and low overhead.

mod xoroshiro128;

pub use xoroshiro128::*;
