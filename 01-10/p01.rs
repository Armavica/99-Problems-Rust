// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 01: Vectors: last
//! 
//! Write a function that returns a reference to the last element of a vector.
//! Your function could have this signature:
//! `fn last<'a, T>(vec: &'a [T]) -> Option<&'a T>`
//!

fn last<'a, T>(vec: &'a [T]) -> Option<&'a T> {
    match vec {
        [] => None,
        [.., ref x] => Some(x)
    }
}

#[test]
fn last_nonempty() {
    let vec = ~['a', 'b', 'c'];
    assert_eq!(last(vec), Some(&'c'));
}

#[test]
fn last_empty() {
    let vec: ~[char] = ~[];
    assert_eq!(last(vec), None);
}

