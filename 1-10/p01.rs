fn last<'a, T>(list: &'a [T]) -> Option<&'a T> {
    match list {
        [] => None,
        [.., ref x] => Some(x)
    }
}

fn main() {
    let list: ~[char] = ~['a', 'b', 'c'];
    println!("{:?}", last(list));
    println!("{:?}", list.last()); // Cheat
}
