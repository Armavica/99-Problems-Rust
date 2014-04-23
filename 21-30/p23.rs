// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 23: Vectors: rnd_select
//!
//! Extract a given number of randomly selected elements from a vector.
//! The selected items shall be returned in a vector.
//!
//! Your function could have this signature:
//! `fn rnd_select<'a, T>(vec: &'a [T], n: uint) -> Vec<&'a T>`

extern crate rand;
use rand::{task_rng, Rng};

fn rnd_select<'a, T>(vec: &'a [T], n: uint) -> Vec<&'a T> {
    task_rng().sample(vec.iter(), n)
}
