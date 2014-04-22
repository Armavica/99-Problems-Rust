// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 20: Vectors: remove_at
//!
//! Return a vector with its `k`th element removed.
//!
//! Your function could have this signature:
//! `fn remove_at<T>(vec: ~[T], k: uint) -> ~[T]`

fn remove_at<T>(vec: ~[T], k: uint) -> ~[T] {
    vec.move_iter()
       .enumerate()
       .filter_map(|(n, e)| if n != k {Some(e)} else {None})
       .collect()
}

#[test]
fn remove_at_in() {
    let list = ~['a', 'b', 'c', 'd'];
    assert_eq!(remove_at(list, 2), ~['a', 'b', 'd']);
}

#[test]
fn remove_at_out() {
    let list = ~['a', 'b', 'c', 'd'];
    assert_eq!(remove_at(list, 10), ~['a', 'b', 'c', 'd']);
}
