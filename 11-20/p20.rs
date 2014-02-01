fn drop<T: Clone>(list: &[T], k: uint) -> ~[T] {
    let mut res = ~[];
    res.push_all(list.slice_to(k-1));
    res.push_all(list.slice_from(k));
    res
}

fn main() {
    let list = ~['a', 'b', 'c', 'd'];
    println!("{:?}", drop(list, 2));
}

