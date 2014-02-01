fn slice<'a, T>(list: &'a [T], i: uint, k: uint) -> &'a [T] {
    list.slice(i-1, k)
}

fn main() {
    let list = ~['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
    println!("{:?}", slice(list, 3, 7));
    println!("{:?}", slice(list, 3, 7));
}

