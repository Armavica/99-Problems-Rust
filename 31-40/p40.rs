// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

/// Problem 40: Arith: goldbach
///
/// Goldbach's conjecture says that every positive even number greater than 2
/// is the sum of two prime numbers.  Example: `28 = 5 + 23`.
/// It is one of the most famous facts in number theory that has not been
/// proved to be correct in the general case. It has been numerically confirmed
/// up to very large numbers. Write a function to find the two prime numbers
/// that sum up to a given even integer.
///
/// Your function could have this signature:
/// `fn goldbach(n: uint) -> Option<(uint, uint)>`

fn primes(start: uint, end: uint) -> ~[uint] {
    let mut primes = vec!(2);
    let mut i = 3;
    loop {
        while primes.iter().any(|&x| i%x == 0) {
            i += 2;
        }
        if i < end {
            primes.push(i)
        } else {
            break
        }
    }
    primes.move_iter().skip_while(|&p| p < start).collect()
}

fn goldbach(n: uint) -> Option<(uint, uint)> {
    let primes = primes(2, n);
    for &p in primes.iter() {
        if primes.bsearch_elem(&(n-p)).is_some() {
            return Some((p, n-p))
        }
    }
    None
}

#[test]
fn goldbach_bench() {
    assert_eq!(goldbach(28), Some((5, 23)))
}


