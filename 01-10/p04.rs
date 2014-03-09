fn length<T>(list: &[T]) -> uint {
    list.len()
}

fn main() {
    let list = ~['a', 'b', 'c'];
    assert!(length(list) == 3);

    let list: ~[uint] = ~[];
    assert!(length(list) == 0);
}
