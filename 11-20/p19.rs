fn rotate<T: Clone>(list: &[T], n: i32) -> ~[T] {
    let mut res = ~[];
    if n >= 0 {
        res.push_all(list.slice_from(n as uint));
        res.push_all(list.slice_to(n as uint));
        res
    } else {
        res.push_all(list.slice_from(list.len() + n as uint));
        res.push_all(list.slice_to(list.len() + n as uint));
        res
    }

}

fn main() {
    let list = ~['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    println!("{:?}", rotate(list, 3));
    println!("{:?}", rotate(list, -2));
}

