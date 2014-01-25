fn main() {
    let list = ~['a', 'b', 'c'];
    println!("{}", list.len());

    let list: ~[uint] = ~[];
    println!("{}", list.len());
}
