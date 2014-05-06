// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

/// Problem 31: Arith: prime
///
/// Determine whether a given integer number is prime.
///
/// Your function could have this signature:
/// `fn is_prime(n: uint) -> bool`
///

fn is_prime(n: uint) -> bool {
    match n {
        0..2 => n == 2,
        _ if n % 2 == 0 => false,
        _ => {
            let mut primes = vec!(2);
            let mut i = 3u;
            while i*i < n {
                while primes.iter().any(|&x| i%x == 0) {
                    i += 2;
                }
                if n % i == 0 {
                    return false;
                }
                primes.push(i);
            }
            true
        }
    }
}

#[test]
fn is_prime_test() {
    assert!(is_prime(104729));
    assert!(!is_prime(302446877));
}

