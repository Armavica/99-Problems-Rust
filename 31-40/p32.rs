// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

/// Problem 32: Arith: gcd
///
/// Determine the greatest common divisor of two positive integer numbers.
///
/// Your function could have this signature:
/// `fn gcd(a: uint, b: uint) -> uint`
///

fn gcd(a: uint, b: uint) -> uint {
    match (a, b) {
        (0, b) => b,
        (a, b) => gcd(b%a, a)
    }
}

#[test]
fn gcd_test() {
    assert_eq!(gcd(13, 27), 1);
    assert_eq!(gcd(20536, 7826), 2);
}


