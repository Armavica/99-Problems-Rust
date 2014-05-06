// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

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


