fn repli<T: Clone>(list: &[T], n: uint) -> ~[T] {
    let mut r = ~[];
    for e in list.iter() {
        for _ in range(0, n) {
            r.push(e.clone());
        }
    }
    r
}

fn main() {
    let list = ~['a', 'b', 'c'];
    println!("{:?}", repli(list, 3));
}

