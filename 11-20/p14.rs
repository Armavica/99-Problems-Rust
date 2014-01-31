fn dupli<T: Clone>(list: &[T]) -> ~[T] {
    let mut r = ~[];
    for e in list.iter() {
        r.push(e.clone());
        r.push(e.clone());
    }
    r
}

fn main() {
    let list = ~['a', 'b', 'c', 'c', 'd'];
    println!("{:?}", dupli(list));
}

