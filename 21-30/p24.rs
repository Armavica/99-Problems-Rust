// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 24: Vectors: lotto_select
//!
//! Lotto: Draw `k` different random numbers from the set `0..n-1`.
//!
//! Your function could have this signature:
//! `fn lotto_select(k: uint, n: uint) -> Vec<uint>`

extern crate rand;
use rand::{task_rng, Rng};

fn lotto_select(k: uint, n: uint) -> Vec<uint> {
    task_rng().sample(range(0, n), k)
}
