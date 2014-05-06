// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

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

