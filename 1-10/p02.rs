fn lastbutone<'a, T>(list: &'a [T]) -> Option<(&'a T, &'a T)> {
    match list {
        [] | [_] => None,
        [.., ref x, ref y] => Some((x, y))
    }
}

fn main() {
    let list = ~['a', 'b', 'c', 'd', 'e'];
    println!("{:?}", lastbutone(list));

    let list = ~['a'];
    println!("{:?}", lastbutone(list));
}
