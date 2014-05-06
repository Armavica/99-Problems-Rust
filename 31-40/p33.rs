// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

/// Problem 33: Arith: coprime
///
/// Determine whether two positive integer numbers are coprime.
///
/// Your function could have this signature:
/// `fn coprime(a: uint, b: uint) -> bool`

extern crate num;

fn coprime(a: uint, b: uint) -> bool {
    num::gcd(a, b) == 1
}

#[test]
fn coprime_test() {
    assert!(coprime(13, 27));
    assert!(!coprime(20536, 7826));
}


