fn drop<T: Clone>(list: &[T], n: uint) -> ~[T] {
    list.iter().enumerate().filter_map(
        |(i,&ref e)|
            if i%n < n-1 {
                Some(e.clone())
            } else {
                None
            }
        ).collect()
}

fn main() {
    let list = ~['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
    println!("{:?}", drop(list, 3));
}

