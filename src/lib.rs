#![doc = include_str!("../README.md")]

pub mod rangeext;
mod requestext;
mod responseext;

pub use crate::{requestext::RequestRangeExt, responseext::ResponseRangeExt};
