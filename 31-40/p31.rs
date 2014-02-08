fn is_prime(n: int) -> bool {
    if n <= 2 {
        return n == 2;
    }
    let mut primes = ~[2];
    let mut i = 3;
    while std::num::pow(i, 2) < n {
        let &p = primes.last().unwrap();
        if n % p == 0 {
            return false;
        }
        while primes.iter().any(|&x| i%x == 0) {
            i += 2;
        }
        primes.push(i);
    }
    true
}

fn main() {
    for n in range(0, 100) {
        if is_prime(n) {
            println!("{}", n);
        }
    }
}


