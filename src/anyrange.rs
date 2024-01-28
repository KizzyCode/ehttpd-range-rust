//! A type-erased inclusive range container

use ehttpd::{error, error::Error};
use std::{
    cmp::Ordering,
    ops::{Bound, RangeBounds, RangeInclusive},
};

/// A type-erased inclusive range
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnyInclusiveRange<T> {
    /// An unbounded/open range
    Full,
    /// A range with a lower inclusive boundary
    From {
        /// The first element (inclusive)
        start: T,
    },
    /// A range with an upper inclusive boundary
    To {
        /// The last element (inclusive)
        end: T,
    },
    /// A range with a lower and upper inclusive boundary
    FromTo {
        /// The first element (inclusive)
        start: T,
        /// The last element (inclusive)
        end: T,
    },
}
impl<T> AnyInclusiveRange<T> {
    /// Creates an inclusive range from `self`, replacing unspecified boundaries with the given boundaries if necessary
    pub fn to_inclusive(&self, start_incl: T, end_incl: T) -> Result<RangeInclusive<T>, Error>
    where
        T: PartialOrd + Copy,
    {
        // Take or specify boundaries
        let start_incl = match self {
            Self::Full | Self::To { .. } => start_incl,
            Self::From { start } => *start,
            Self::FromTo { start, .. } => *start,
        };
        let end_incl = match self {
            Self::Full | Self::From { .. } => end_incl,
            Self::To { end } => *end,
            Self::FromTo { end, .. } => *end,
        };

        // Validate resulting range
        match start_incl.partial_cmp(&end_incl) {
            Some(Ordering::Less | Ordering::Equal) => Ok(start_incl..=end_incl),
            _ => Err(error!("End of inclusive range is before start")),
        }
    }

    /// Converts the range into a different type
    pub fn convert<U, E>(self) -> Result<AnyInclusiveRange<U>, E>
    where
        T: TryInto<U>,
        E: From<<T as TryInto<U>>::Error>,
    {
        let range = match self {
            Self::Full => AnyInclusiveRange::Full,
            Self::From { start } => AnyInclusiveRange::From { start: start.try_into()? },
            Self::To { end } => AnyInclusiveRange::To { end: end.try_into()? },
            Self::FromTo { start, end } => AnyInclusiveRange::FromTo { start: start.try_into()?, end: end.try_into()? },
        };
        Ok(range)
    }
}
impl<T> RangeBounds<T> for AnyInclusiveRange<T> {
    fn start_bound(&self) -> Bound<&T> {
        match self {
            Self::Full | Self::To { .. } => Bound::Unbounded,
            Self::From { start } | Self::FromTo { start, .. } => Bound::Included(start),
        }
    }
    fn end_bound(&self) -> Bound<&T> {
        match self {
            AnyInclusiveRange::Full | AnyInclusiveRange::From { .. } => Bound::Unbounded,
            AnyInclusiveRange::To { end } | AnyInclusiveRange::FromTo { end, .. } => Bound::Included(end),
        }
    }
}
