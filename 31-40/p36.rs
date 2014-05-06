// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

/// Problem 36: Arith: factors_mul
///
/// Determine the prime factors of a given positive integer and their
/// multiplicity.
/// Construct a list containing the prime factors and their multiplicity.
///
/// Your function could have this signature:
/// `fn factors_mul(n: uint) -> ~[(uint, uint)]`

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

#[test]
fn factors_mul_test() {
    assert_eq!(factors_mul(315), ~[(3, 2), (5, 1), (7, 1)]);
}


