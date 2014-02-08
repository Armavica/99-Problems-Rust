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

fn goldbach_list(start: uint, end: uint) -> ~[(uint, Option<(uint, uint)>)] {
    range(start, end+1).filter_map(
        |x|
        if x%2 == 0 {
            Some((x, goldbach(x)))
        } else {
            None
        }
        ).to_owned_vec()
}

fn goldbach_limit(start: uint, end: uint, limit: uint) -> ~[(uint, Option<(uint, uint)>)] {
    goldbach_list(start, end).move_iter().filter(
        |&x|
        match x {
            (_, Some((a, b))) if a > limit && b > limit => true,
            (_, Some(_)) => false,
            (n, None)    => fail!(format!("Goldbach conjecture is false for {}", n))
        }
        ).to_owned_vec()
}



fn main() {
    println!("{:?}", goldbach_list(9, 20));
    println!("{:?}", goldbach_limit(3, 2000, 50));
}


