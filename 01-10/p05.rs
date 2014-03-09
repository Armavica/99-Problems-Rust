fn rev<T>(list: ~[T]) -> ~[T] {
    list.move_rev_iter().collect()
}

fn main() {
    let list = ~['a', 'b', 'c', 'd', 'e'];
    assert!(rev(list) == ~['e', 'd', 'c', 'b', 'a']);
}
