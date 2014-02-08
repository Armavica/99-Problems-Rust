use std::iter::MultiplicativeIterator;

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


fn main() {
    println!("{:?}", phi_improved(10));
    println!("{:?}", phi_improved(13));
}


