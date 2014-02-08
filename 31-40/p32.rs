fn gcd(a: int, b: int) -> int {
    match (a, b) {
        (0, b) => b,
        (a, b) => gcd(b%a, a)
    }
}

fn main() {
    println!("{} {}", gcd(13, 27), std::num::gcd(13, 27));
    println!("{} {}", gcd(20536, 7826), std::num::gcd(20536, 7826));
}


