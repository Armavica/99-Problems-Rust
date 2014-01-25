fn is_palindrome<T: Eq>(list: &~[T]) -> bool {
    let mut comp = list.iter().zip(list.rev_iter());
    for (a, b) in comp {
        if *a != *b {
            return false
        }
    }
    true
}

fn main() {
    let list = ~['x', 'a', 'm', 'a', 'x'];
    println!("{:?}", is_palindrome(&list));

    let list = ~['a', 'b'];
    println!("{:?}", is_palindrome(&list));
}
