// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

/// Problem 34: Arith: phi
///
/// Compute Euler's totient function phi.
///
/// Your function could have this signature:
/// `fn phi(n: uint) -> uint`

extern crate test;
extern crate num;

fn coprime(a: uint, b: uint) -> bool {
    num::gcd(a, b) == 1
}

fn phi(n: uint) -> uint {
    range(1, n).filter(|&i| coprime(n, i)).len()
}

#[test]
fn phi_test() {
    assert_eq!(phi(10), 4);
    assert_eq!(phi(13), 12);
}

#[bench]
fn phi_bench(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(phi(10090), 4032))
}

