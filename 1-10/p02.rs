fn lastbutone<'a, T>(list: &'a ~[T]) -> Option<(&'a T, &'a T)> {
    if list.len() > 2 {
        Some((&list[list.len()-2], &list[list.len()-1]))
    } else {
        None
    }
}

fn main() {
    let list = ~['a', 'b', 'c', 'd', 'e'];
    println!("{:?}", lastbutone(&list));

    let list = ~['a'];
    println!("{:?}", lastbutone(&list));
}
