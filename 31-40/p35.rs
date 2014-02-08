fn factors(n: uint) -> ~[uint] {
    if n <= 1 {
        return ~[n];
    }
    let mut primes = ~[2];
    let mut result = ~[];
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
    result
}

fn main() {
    println!("{:?}", factors(315));
}


