use std::rand::{task_rng, Rng};

fn rnd_permu<T>(list: ~[T]) -> ~[T] {
    task_rng().shuffle(list)
}

fn main() {
    let list = ~['a', 'b', 'c', 'd', 'e', 'f'];
    println!("{:?}", rnd_permu(list));
}

