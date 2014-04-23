// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 21: Vectors: insert
//!
//! Insert an element at a given position into a vector.  Start counting vector
//! elements with 0. If the position is larger or equal to the length of the
//! vector, insert the element at the end.
//!
//! Your function could have this signature:
//! `fn insert<T>(elem: T, vec: Vec<T>, k: uint) -> Vec<T>`

use std::cmp::min;

fn insert<T>(elem: T, vec: Vec<T>, k: uint) -> Vec<T> {
    let mut vec = vec;
    let l = vec.len();
    vec.insert(min(k, l), elem);
    vec
}

#[test]
fn insert_in() {
    let vec = vec!('a', 'b', 'c', 'd');
    assert_eq!(insert('X', vec, 2), vec!('a', 'b', 'X', 'c', 'd'));
}

#[test]
fn insert_out() {
    let vec = vec!('a', 'b', 'c', 'd');
    assert_eq!(insert('X', vec, 10), vec!('a', 'b', 'c', 'd', 'X'));
}

