// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 28: Vectors: sorts
//!
//! We suppose that a vector contains element that are vectors themselves.
//! Write a function `length_sort` which sorts the elements of this vector
//! according to their length.
//!
//! Your function could have this signature:
//! `fn length_sort<T>(vec: ~[~[T]]) -> ~[~[T]]`
//!
//! Now write a function `frequenry_sort` which sorts the elements of this
//! vector according to their length frequency in the vector: i.e. vector with
//! rare lengths are placed first, others with a more frequent length come
//! later.
//! 
//! Your function could have this signature:
//! `fn frequency_sort<T>(vec: ~[~[T]]) -> ~[~[T]]`

extern crate collections;
use collections::hashmap::HashMap;

fn length_sort<T>(vec: ~[~[T]]) -> ~[~[T]] {
    let mut vec = vec;
    vec.sort_by(|a, b| a.len().cmp(&b.len()));
    vec
}

fn frequency_sort<T>(vec: ~[~[T]]) -> ~[~[T]] {
    let mut freqs = HashMap::<uint, uint>::new();
    for v in vec.iter() {
        freqs.insert_or_update_with(v.len(), 1, |_, n| *n += 1);
    }
    let mut vec = vec;
    vec.sort_by(|a, b| freqs.get(&a.len()).cmp(freqs.get(&b.len())));
    vec
}

#[test]
fn length_test() {
    let vec = ~[~['a', 'b', 'c'], ~['d', 'e'], ~['f', 'g', 'h'],
                ~['d', 'e'], ~['i', 'j', 'k', 'l'], ~['m', 'n'], ~['o']];
    assert_eq!(length_sort(vec),
            ~[~['o'],
              ~['d', 'e'], ~['d', 'e'], ~['m', 'n'],
              ~['a', 'b', 'c'], ~['f', 'g', 'h'],
              ~['i', 'j', 'k', 'l']])
}

#[test]
fn frequency_test() {
    let vec = ~[~['a', 'b', 'c'], ~['d', 'e'], ~['f', 'g', 'h'],
                ~['d', 'e'], ~['i', 'j', 'k', 'l'], ~['m', 'n'], ~['o']];
    assert_eq!(frequency_sort(vec),
            ~[~['i', 'j', 'k', 'l'], ~['o'],
              ~['a', 'b', 'c'], ~['f', 'g', 'h'],
              ~['d', 'e'], ~['d', 'e'], ~['m', 'n']])
}
