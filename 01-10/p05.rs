// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 05: Vectors: reverse
//!
//! Reverse a vector.
//!
//! Your function must have this signature:
//! `fn rev<T>(vector: ~[T]) -> ~[T]`
//!

fn rev<T>(vector: ~[T]) -> ~[T] {
    vector.move_rev_iter().collect()
}

#[test]
fn test05_rev() {
    let vector = ~['a', 'b', 'c', 'd', 'e'];
    assert!(rev(vector) == ~['e', 'd', 'c', 'b', 'a']);
}

