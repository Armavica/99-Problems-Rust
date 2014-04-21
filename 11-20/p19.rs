// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 19: Vectors: rotate
//!
//! Rotate a vector N places to the left.
//!
//! Your function could have this signature:
//! `fn rotate<T>(vec: ~[T], n: int) -> ~[T]`

fn rotate<T: Clone>(vec: ~[T], n: int) -> ~[T] {
    let mut res = ~[];
    let l = vec.len() as int;
    let r: uint = (((n % l) + l) % l) as uint;

    res.push_all(vec.slice_from(r));
    res.push_all(vec.slice_to(r));
    res
}

#[test]
fn rotate_left() {
    let vec = ~['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    assert_eq!(rotate(vec, 3), ~['d', 'e', 'f', 'g', 'h', 'a', 'b', 'c']);
}

#[test]
fn rotate_right() {
    let vec = ~['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    assert_eq!(rotate(vec, -2), ~['g', 'h', 'a', 'b', 'c', 'd', 'e', 'f']);
}

#[test]
fn rotate_mod() {
    let vec = ~['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    assert_eq!(rotate(vec, -10), ~['g', 'h', 'a', 'b', 'c', 'd', 'e', 'f']);
}

