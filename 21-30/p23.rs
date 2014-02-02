use std::rand::{task_rng, Rng};

fn rnd_select<'a, T>(list: &'a [T], n: uint) -> ~[&'a T] {
    task_rng().sample(list.iter(), n)
}

fn main() {
    let list = ~['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    println!("{:?}", rnd_select(list, 3));
}

