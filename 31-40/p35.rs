// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

/// Problem 35: Arith: factors
///
/// Determine the prime factors of a given positive integer.
/// Construct a flat list containing the prime factors in ascending order.
///
/// Your function could have this signature:
/// `fn factors(n: uint) -> ~[uint]`

fn factors(n: uint) -> ~[uint] {
    if n <= 1 {
        return ~[n];
    }
    let mut primes = vec!(2);
    let mut result = Vec::new();
    let mut i = 3;
    let mut n = n;
    while n != 1 {
        let &p = primes.last().unwrap();
        while n % p == 0 {
            result.push(p);
            n /= p;
        }
        while primes.iter().any(|&x| i%x == 0) {
            i += 2;
        }
        primes.push(i);
    }
    result.as_slice().to_owned()
}

#[test]
fn factors_test() {
    assert_eq!(factors(315), ~[3, 3, 5, 7]);
}


