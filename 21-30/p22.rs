// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 22: Vectors: range
//!
//! Create a vector containing all integers within a given range.  If the first
//! argument is smaller than the second, produce a vector in decreasing order.
//!
//! Your function could have this signature:
//! `fn rotate(from: int, to: int) -> ~[int]`

fn rang(from: int, to: int) -> ~[int] {
    if to >= from {
        range(from, to).collect()
    } else {
        range(to+1, from+1).rev().collect()
    }
}

#[test]
fn range_increasing() {
    assert_eq!(rang(4, 9), ~[4, 5, 6, 7, 8]);
}

#[test]
fn range_decreasing() {
    assert_eq!(rang(9, 4), ~[9, 8, 7, 6, 5]);
}

