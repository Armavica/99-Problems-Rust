fn primes(start: uint, end: uint) -> ~[uint] {
    let mut primes = ~[2];
    let mut i = 3;
    while i < end {
        while primes.iter().any(|&x| i%x == 0) {
            i += 2;
        }
        primes.push(i);
    }
    primes.move_iter().filter(|&p| p >= start && p < end).to_owned_vec()
}

fn goldbach(n: uint) -> Option<(uint, uint)> {
    let primes = primes(2, n);
    for &p in primes.iter() {
        match primes.bsearch_elem(&(n-p)) {
            Some(_) => return Some((p, n-p)),
            None => {}
        }
    }
    None
}

fn main() {
    println!("{:?}", goldbach(28));
}


