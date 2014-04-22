// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 17: Vectors: split
//!
//! Split a vector into two parts; the length of the first part is given.
//! If the length of the first part is longer than the entire vec, then the
//! first part is the vec and the second part is empty.
//!
//! Your function could have this signature:
//! `fn split<T>(vec: &[T], n: uint) -> (&[T], &[T])`

use std::cmp::min;

fn split<'a, T>(vec: &'a [T], n: uint) -> (&'a [T], &'a [T]) {
    (vec.slice_to(min(n, vec.len())), vec.slice_from(min(n, vec.len())))
}

#[test]
fn split_ok() {
    let vec = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
    assert_eq!(split(vec, 3),
        (&['a', 'b', 'c'], &['d', 'e', 'f', 'g', 'h', 'i', 'j']));
}

#[test]
fn split_begin() {
    let vec = ['a', 'b', 'c', 'd', 'e'];
    assert_eq!(split(vec, 0),
        (&[], &['a', 'b', 'c', 'd', 'e']));
}

#[test]
fn split_end() {
    let vec = ['a', 'b', 'c', 'd', 'e'];
    assert_eq!(split(vec, 5),
        (&['a', 'b', 'c', 'd', 'e'], &[]));
}

#[test]
fn split_longer() {
    let vec = ['a', 'b', 'c', 'd', 'e'];
    assert_eq!(split(vec, 10),
        (&['a', 'b', 'c', 'd', 'e'], &[]));
}
