use std::rand::{task_rng, Rng};

fn lotto_select(n: uint, m: uint) -> ~[uint] {
    task_rng().sample(range(1, m+1), n)
}

fn main() {
    println!("{:?}", lotto_select(6, 49));
}

