// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 25: Vectors: rnd_permu
//!
//! Generate a random permutation of the elements of a vector.
//!
//! Your function could have this signature:
//! `fn rnd_permu<T>(vec: ~[T]) -> Vec<T>`

extern crate rand;
use rand::{task_rng, Rng};

fn rnd_permu<T>(vec: ~[T]) -> ~[T] {
    let mut vec = vec;
    task_rng().shuffle(vec);
    vec
}
