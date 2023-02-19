use ehttpd_range::rangeext::RangeExt;
use std::ops::{Range, RangeInclusive};

#[test]
fn to_inclusive() {
    // Test full range
    let range = RangeInclusive::<u64>::from_range_bounds(.., 0, 1).expect("failed to convert range");
    assert_eq!(range, 0..=0);

    // Test half-open range
    let range = RangeInclusive::<u64>::from_range_bounds(0.., 0, 1).expect("failed to convert range");
    assert_eq!(range, 0..=0);

    // Test half-closed range
    let range = RangeInclusive::<u64>::from_range_bounds(..0, 0, 1).expect("failed to convert range");
    assert_eq!(range, 0..=0);

    // Test large range
    let range = RangeInclusive::<u64>::from_range_bounds(7.., 0, u64::MAX).expect("failed to convert range");
    assert_eq!(range, 7..=(u64::MAX - 1));
}

#[test]
fn to_exclusive() {
    // Test full range
    let range = Range::<u64>::from_range_bounds(.., 0, 0).expect("failed to convert range");
    assert_eq!(range, 0..0);

    // Test half-open range
    let range = Range::<u64>::from_range_bounds(0.., 0, 0).expect("failed to convert range");
    assert_eq!(range, 0..0);

    // Test half-closed range
    let range = Range::<u64>::from_range_bounds(..0, 0, 0).expect("failed to convert range");
    assert_eq!(range, 0..0);

    // Test large range
    let range = Range::<u64>::from_range_bounds(7.., 0, u64::MAX).expect("failed to convert range");
    assert_eq!(range, 7..u64::MAX);
}
