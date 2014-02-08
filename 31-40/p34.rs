fn gcd(a: int, b: int) -> int {
    match (a, b) {
        (0, b) => b,
        (a, b) => gcd(b%a, a)
    }
}

fn coprime(a: int, b: int) -> bool {
    gcd(a, b) == 1
}

fn phi(n: int) -> uint {
    range(1, n).filter(|&i| coprime(n, i)).len()
}

fn main() {
    println!("{}", phi(10));
    println!("{}", phi(13));
}


