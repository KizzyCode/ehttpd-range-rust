//! Utilities to work with range bounds and ranges

use std::ops::{Bound, Range, RangeBounds, RangeInclusive};

/// An extension trait for ranges
pub trait RangeExt<T>
where
    Self: Sized,
{
    /// Creates `Self` from the given range bounds and validates that it is within `min_incl` and `max_excl`
    fn from_range_bounds<B>(bounds: B, min_incl: T, max_excl: T) -> Option<Self>
    where
        B: RangeBounds<T>;
}

/// Implements `RangeExt` for `RangeInclusive` with the given unsized integer
macro_rules! impl_rangeext_range_uint {
    ($uint:ty) => {
        impl RangeExt<$uint> for Range<$uint> {
            fn from_range_bounds<B>(bounds: B, min_incl: $uint, max_excl: $uint) -> Option<Self>
            where
                B: RangeBounds<$uint>,
            {
                // Compute the bounds
                let start = match bounds.start_bound() {
                    Bound::Included(start) => *start,
                    Bound::Excluded(_) => unreachable!("excluded bounds are invalid for range starts"),
                    Bound::Unbounded => min_incl,
                };
                let end = match bounds.end_bound() {
                    Bound::Included(before_end) => before_end.saturating_add(1),
                    Bound::Excluded(end) => *end,
                    Bound::Unbounded => max_excl,
                };

                // Validate the range
                if start < min_incl || start > max_excl {
                    return None;
                }
                if end < min_incl || end > max_excl {
                    return None;
                }
                Some(start..end)
            }
        }
    };
}
impl_rangeext_range_uint!(u64);
impl_rangeext_range_uint!(usize);

/// Implements `RangeExt` for `RangeInclusive` with the given unsized integer
macro_rules! impl_rangeext_rangeinclusive_uint {
    ($uint:ty) => {
        impl RangeExt<$uint> for RangeInclusive<$uint> {
            fn from_range_bounds<B>(bounds: B, min_incl: $uint, max_excl: $uint) -> Option<Self>
            where
                B: RangeBounds<$uint>,
            {
                // Compute the bounds
                let start = match bounds.start_bound() {
                    Bound::Included(start) => *start,
                    Bound::Excluded(_) => unreachable!("excluded bounds are invalid for range starts"),
                    Bound::Unbounded => min_incl,
                };
                let end_incl = match bounds.end_bound() {
                    Bound::Included(end) => *end,
                    Bound::Excluded(after_end) => after_end.saturating_sub(1),
                    Bound::Unbounded => max_excl.saturating_sub(1),
                };

                // Validate the range
                if start < min_incl || start >= max_excl {
                    return None;
                }
                if end_incl < min_incl || end_incl >= max_excl {
                    return None;
                }
                Some(start..=end_incl)
            }
        }
    };
}
impl_rangeext_rangeinclusive_uint!(u64);
impl_rangeext_rangeinclusive_uint!(usize);
