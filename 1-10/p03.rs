enum List<T> {
    Cons(T, ~List<T>),
    Nil
}

fn kth<'a, T>(k: uint, list: &'a List<T>) -> Option<&'a T> {
    match *list {
        Nil => None,
        Cons(ref elem, _) if k == 1 => Some(elem),
        Cons(_, ~ref rest) => kth(k-1, rest)
    }
}

fn main() {
    let list: List<char> = Cons('a', ~Cons('b', ~Cons('c', ~Cons('d', ~Nil))));
    println!("{:?}", kth(3, &list));
    println!("{:?}", kth(7, &list));
}
