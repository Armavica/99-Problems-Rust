fn rev<T: Clone>(list: &~[T]) -> ~[T] {
    let mut tsli = ~[];
    for elem in list.iter() {
        tsli.unshift(elem.clone());
    }
    tsli
}

fn main() {
    let list = ~['a', 'b', 'c', 'd', 'e'];
    println!("{:?}", rev(&list));
}
