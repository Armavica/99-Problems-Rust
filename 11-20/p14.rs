// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 14: Vectors: duplicate
//!
//! Duplicate the elements of a vector.
//!
//! Your function could have this signature:
//! `fn dupli<T: Clone>(vec: ~[T]) -> ~[T]`

fn dupli<T: Clone>(vec: ~[T]) -> ~[T] {
    vec.move_iter().flat_map(|e| Vec::from_elem(2, e).move_iter()).collect()
}

#[test]
fn dupli_test() {
    let vec = ~['a', 'b', 'c', 'c', 'd'];
    assert_eq!(dupli(vec), ~['a', 'a', 'b', 'b', 'c', 'c', 'c', 'c', 'd', 'd']);
}
