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
fn test01_last() {
    let vector = ['a', 'b', 'c'];
    let empty: &[char] = [];

    assert!(last(vector) == Some(&'c'));
    assert!(vector.last() == Some(&'c'));
    assert!(last(empty) == None);
    assert!(empty.last() == None);
}

