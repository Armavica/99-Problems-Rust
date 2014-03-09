fn kth<'a, T>(k:uint, list: &'a [T]) -> Option<&'a T> {
    if list.len() >= k && k > 0{
        Some(&list[k-1])
    } else {
        None
    }
}

fn main() {
    let list = ~['a', 'b', 'c', 'd', 'e'];
    assert!(kth(3, list) == Some(&'c'));

    let list = ~['a'];
    assert!(kth(3, list) == None)
}
