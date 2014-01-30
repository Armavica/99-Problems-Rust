fn main() {
    let mut list =
        ~['a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e'];

    list.dedup(); // Cheat
    println!("{:?}", list);
}
