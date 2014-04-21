// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 15: Vectors: replicate
//!
//! Replicate the elements of a vector a given number of times.
//!
//! Your function could have this signature:
//! `fn repli<T: Clone>(n: uint, vec: ~[T]) -> ~[T]`

fn repli<T: Clone>(n: uint, vec: ~[T]) -> ~[T] {
    vec.move_iter().flat_map(|e| Vec::from_elem(n, e).move_iter()).collect()
}

#[test]
fn dupli_0() {
    let vec = ~['a', 'b', 'c', 'c', 'd'];
    assert_eq!(repli(0, vec), ~[]);
}

#[test]
fn dupli_2() {
    let vec = ~['a', 'b', 'c', 'c', 'd'];
    assert_eq!(repli(2, vec), ~['a', 'a', 'b', 'b', 'c', 'c', 'c', 'c', 'd', 'd']);
}
