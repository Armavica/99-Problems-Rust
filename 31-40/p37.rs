// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

/// Problem 37: Arith: phi_improved
///
/// If the list of prime factors of a number `m` is known in the form of its
/// prime numbers together with their multiplicity, then the function `phi(m)`
/// can be efficiently computed as follows:
/// Let `[(p1, m1), (p2, m2), (p3, m3), ...]` be the list of prime factors (and
/// their multiplicities) of a given number `m`.  Then `phi(m)` can be
/// calculated with the following formula:
/// ```
/// phi(m) = (p1-1)*p1**(m1-1) + (p2-1)*p2**(m2-1) + (p3-1)*p3**(m3-1) + ...
/// ```
///
/// Your function could have this signature:
/// `fn phi_improved(n: uint) -> uint`

extern crate test;
use std::iter::MultiplicativeIterator;

fn factors_mul(n: uint) -> ~[(uint, uint)] {
    if n <= 1 {
        return ~[(n, 1)];
    }
    let mut primes = vec!(2);
    let mut result = Vec::new();
    let mut i = 3;
    let mut n = n;
    while n != 1 {
        let &p = primes.last().unwrap();
        let mut j = 0u;
        while n % p == 0 {
            j += 1;
            n /= p;
        }
        if j > 0 {
            result.push((p, j));
        }
        while primes.iter().any(|&x| i%x == 0) {
            i += 2;
        }
        primes.push(i);
    }
    result.as_slice().to_owned()
}

fn phi_improved(n: uint) -> uint {
    factors_mul(n).iter().map(|&(p, m)| (p-1)*std::num::pow(p, m-1)).product()
}

#[test]
fn phi_improved_test() {
    assert_eq!(phi_improved(10), 4);
    assert_eq!(phi_improved(13), 12);
}

#[bench]
fn phi_improved_bench(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(phi_improved(10090), 4032))
}

