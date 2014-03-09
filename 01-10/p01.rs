fn last<'a, T>(list: &'a [T]) -> Option<&'a T> {
    match list {
        [] => None,
        [.., ref x] => Some(x)
    }
}

fn main() {
    let list = ['a', 'b', 'c'];
    assert!(last(list) == Some(&'c'));
    assert!(list.last() == Some(&'c')); // Cheat
}
