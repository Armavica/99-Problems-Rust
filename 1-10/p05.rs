fn main() {
    let list = ~['a', 'b', 'c', 'd', 'e'];
    let l:~[&char] = list.rev_iter().collect(); // Cheat
    println!("{:?}", l);
}
