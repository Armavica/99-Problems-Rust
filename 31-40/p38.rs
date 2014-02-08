// Compile with `rustc p38.rs --test` and execute with `./p38 --bench`
extern mod extra;
use std::iter::MultiplicativeIterator;

fn gcd(a: uint, b: uint) -> uint {
    match (a, b) {
        (0, b) => b,
        (a, b) => gcd(b%a, a)
    }
}

fn coprime(a: uint, b: uint) -> bool {
    gcd(a, b) == 1
}

fn phi(n: uint) -> uint {
    range(1, n).filter(|&i| coprime(n, i)).len()
}

fn factors(n: uint) -> ~[(uint, uint)] {
    if n <= 1 {
        return ~[(n, 1)];
    }
    let mut primes = ~[2];
    let mut result = ~[];
    let mut i = 3;
    let mut n = n;
    while n != 1 {
        let &p = primes.last().unwrap();
        let mut j = 0;
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
    result
}

fn phi_improved(n: uint) -> uint {
    factors(n).iter().map(|&(p, m)| (p-1)*std::num::pow(p, m-1)).product()
}

#[bench]
fn compute_phi(b: &mut extra::test::BenchHarness) {
    b.iter(|| {phi(10090);});
}

#[bench]
fn compute_phi_improved(b: &mut extra::test::BenchHarness) {
    b.iter(|| {phi_improved(10090);});
}
