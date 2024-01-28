#![doc = include_str!("../README.md")]

pub mod anyrange;
pub mod rangeext;
mod requestext;
mod responseext;

pub use crate::{requestext::RequestRangeExt, responseext::ResponseRangeExt};

// Re-export our ehttpd dependency
pub use ehttpd;
