// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 08: Vectors: compress
//!
//! Eliminate consecutive duplicates of vector elements.
//!
//! Your function could have this signature:
//! `fn compress<T: Eq>(vec: ~[T]) -> ~[T]`
//!

fn compress_rec<T: Eq>(vec: ~[T]) -> ~[T] {
    let mut result = ~[];
    for elem in vec.move_iter() {
        if result.last().is_none() || result.last().unwrap() != &elem {
            result.push(elem)
        }
    }
    result
}

fn compress_lib<T: Eq>(vec: ~[T]) -> ~[T] {
    let mut r = vec;
    r.dedup();
    r
}

#[test]
fn test08_compress() {
    let vector = ~['a', 'a', 'a', 'a', 'b', 'c', 'c',
    'a', 'a', 'd', 'e', 'e', 'e', 'e'];

    assert!(compress(vector) == ~['a', 'b', 'c', 'a', 'd', 'e']);
}

