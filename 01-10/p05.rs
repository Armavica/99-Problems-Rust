// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 05: Vectors: reverse
//!
//! Reverse a vector.
//!
//! Your function could have this signature:
//! `fn reverse<T>(vec: ~[T]) -> ~[T]`
//!

fn reverse<T>(vec: ~[T]) -> ~[T] {
    vec.move_rev_iter().collect()
}

#[test]
fn reverse_nonempty() {
    let vec = ~['a', 'b', 'c', 'd', 'e'];
    assert_eq!(reverse(vec), ~['e', 'd', 'c', 'b', 'a']);
}

#[test]
fn reverse_empty() {
    let vec: ~[char] = ~[];
    assert_eq!(reverse(vec), ~[]);
}

