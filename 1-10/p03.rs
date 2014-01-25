fn kth<'a, T>(k:uint, list: &'a ~[T]) -> Option<&'a T> {
    if list.len() >= k && k > 0{
        Some(&list[k-1])
    } else {
        None
    }
}

fn main() {
    let list = ~['a', 'b', 'c', 'd', 'e'];
    println!("{:?}", kth(3, &list));

    let list = ~['a'];
    println!("{:?}", kth(3, &list));
}
