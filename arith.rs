// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

extern crate num;
extern crate test;
use std::iter::MultiplicativeIterator;

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



/// Problem 33: Arith: coprime
///
/// Determine whether two positive integer numbers are coprime.
///
/// Your function could have this signature:
/// `fn coprime(a: uint, b: uint) -> bool`

fn coprime(a: uint, b: uint) -> bool {
    num::gcd(a, b) == 1
}

#[test]
fn coprime_test() {
    assert!(coprime(13, 27));
    assert!(!coprime(20536, 7826));
}



/// Problem 34: Arith: phi
///
/// Compute Euler's totient function phi.
///
/// Your function could have this signature:
/// `fn phi(n: uint) -> uint`

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

/// Problem 39: Arith: primes
///
/// Given a range of integers by its lower and upper limit, construct a list of
/// all prime numbers in that range.
///
/// Your function could have this signature:
/// `fn primes(start: uint, end: uint) -> ~[uint]`

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

#[test]
fn primes_test() {
    assert_eq!(primes(10000, 20000).len(), 1033);
}



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



/// Problem 41: Arith: goldbach_limit
///
/// Given a range of integers by its lower and upper limit, print a list of all
/// even numbers and their Goldbach composition.
///
/// Your function could have this signature:
/// `fn goldbach_list(start: uint, end: uint) -> Vec<(uint, Option<(uint, uint)>)>`
///
/// In most cases, if an even number is written as the sum of two prime
/// numbers, one of them is very small.  Very rarely, the primes are both bigger
/// than say 50.  Try to find out how many such cases there are in the range
/// 2..2000.
///
/// Your function could have this signature:
/// `fn goldbach_limit(start: uint, end: uint, limit: uint) -> uint`

fn goldbach_list(start: uint, end: uint) -> Vec<(uint, Option<(uint, uint)>)> {
    std::iter::range_step(start + start%2, end, 2).map(|x| (x, goldbach(x))).collect()
}

fn goldbach_limit(start: uint, end: uint, limit: uint) -> uint {
    goldbach_list(start, end).move_iter().count(
        |x|
        match x {
            (_, Some((a, b))) if a > limit && b > limit => true,
            (_, Some(_)) => false,
            (n, None)    => fail!(format!("Goldbach conjecture is false for {}", n))
        }
        )
}



#[test]
fn golbach_list_test() {
    assert_eq!(goldbach_list(9, 20),
        vec!((10, Some((3, 7))), (12, Some((5, 7))), (14, Some((3, 11))),
             (16, Some((3, 13))), (18, Some((5, 13)))))
}

#[test]
fn goldbach_limit_test() {
    assert_eq!(goldbach_limit(3, 2000, 50), 4);
}

