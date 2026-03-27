#![doc = include_str!("../README.md")]

pub mod anyrange;
pub mod rangeext;
mod rangerequest;
mod rangeresponse;

pub use crate::rangerequest::RangeRequest;
pub use crate::rangeresponse::RangeResponse;
// Re-export our ehttpd dependency
pub use ehttpd;
