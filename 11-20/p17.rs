fn split<'a, T>(list: &'a [T], n: uint) -> (&'a [T], &'a [T]) {
    (list.slice_to(n), list.slice_from(n))
}

fn main() {
    let list = ~['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
    println!("{:?}", split(list, 3));
}

