// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 18: Vectors: slice
//!
//! Given two indices, `i` and `k`, the slice is the vector containing the
//! elements between the `i`th and `k`th element of the original vector (first
//! limit included only).  Start counting the elements with 0.  Your function
//! should not fail in case one or the other indices point outside the vector.
//!
//! Your function could have this signature:
//! `fn dupli<T>(vec: &'a [T], i: uint, k: uint) -> &'a [T]`

use std::cmp::min;

fn slice<'a, T>(vec: &'a [T], i: uint, k: uint) -> &'a [T] {
    vec.slice(min(i, vec.len()), min(k, vec.len()))
}

#[test]
fn slice_ok() {
    let vec = ~['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
    assert_eq!(slice(vec, 3, 7), &['d', 'e', 'f', 'g']);
}

#[test]
fn slice_outside() {
    let vec = ~['a', 'b', 'c', 'd', 'e'];
    assert_eq!(slice(vec, 7, 10), &[]);
}

