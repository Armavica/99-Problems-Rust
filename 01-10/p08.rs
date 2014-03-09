fn compress<T: Eq>(list: ~[T]) -> ~[T] {
    let mut list = list;
    list.dedup();
    list
}


fn main() {
    let list =
        ~['a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e'];

    assert!(compress(list) == ~['a', 'b', 'c', 'a', 'd', 'e']);
}
