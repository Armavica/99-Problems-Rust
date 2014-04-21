// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 16: Vectors: drop
//!
//! Drop every `n`th element of a vector.
//!
//! Your function could have this signature:
//! `fn drop<T>(n: uint, vec: ~[T]) -> ~[T]`

fn drop<T>(vec: ~[T], n: uint) -> ~[T] {
    vec.move_iter().enumerate().filter_map(
        |(i,e)|
            if i%n < n-1 {
                Some(e)
            } else {
                None
            }
        ).collect()
}

#[test]
fn drop_test() {
    let vec = ~['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
    assert_eq!(drop(vec, 3), ~['a', 'b', 'd', 'e', 'g', 'h', 'j']);
}

