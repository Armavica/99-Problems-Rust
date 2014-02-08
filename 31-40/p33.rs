fn gcd(a: int, b: int) -> int {
    match (a, b) {
        (0, b) => b,
        (a, b) => gcd(b%a, a)
    }
}

fn coprime(a: int, b: int) -> bool {
    gcd(a, b) == 1
}

fn main() {
    assert!(coprime(13, 27));
    assert!(!coprime(20536, 7826));
}


