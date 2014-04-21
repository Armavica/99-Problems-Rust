// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 02: Vectors: last but one
//!
//! Find the last but one (last and penultimate) elements of a vector.
//! Your function could have this signature:
//! `fn last_but_one<'a, T>(vec: &'a [T]) -> Option<(&'a T, &'a T)>`
//!

fn last_but_one<'a, T>(vector: &'a [T]) -> Option<(&'a T, &'a T)> {
    match vector {
        [] | [_] => None,
        [.., ref x, ref y] => Some((x, y))
    }
}

#[test]
fn test02_last_but_one() {
    let vector = ['a', 'b', 'c', 'd', 'e'];
    assert!(last_but_one(vector) == Some((&'d', &'e')));

    let vector = ['a'];
    assert!(last_but_one(vector) == None);
}

