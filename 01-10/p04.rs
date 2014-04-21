// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 04: Vectors: length
//!
//! Compute the number of elements of a vector.
//!
//! Your function could have this signature:
//! `fn length<T>(vec: &[T]) -> uint`
//!

fn length<T>(vec: &[T]) -> uint {
    vec.len()
}

#[test]
fn length_nonempty() {
    let vec = ~['a', 'b', 'c'];
    assert_eq!(length(vec), 3);
}

#[test]
fn length_empty() {
    let vec: ~[uint] = ~[];
    assert_eq!(length(vec), 0);
}

