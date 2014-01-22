enum List<T> {
    Cons(T, ~List<T>),
    Nil
}

fn lastbutone<'a, T>(list: &'a List<T>) -> Option<(&'a T, &'a T)> {
    match *list {
        Nil => None,
        Cons(_, ~Nil) => None,
        Cons(ref butone, ~Cons(ref last, ~Nil)) => Some((butone, last)),
        Cons(_, ~ref rest) => lastbutone(rest)
    }
}

fn main() {
    let list: List<char> = Cons('a', ~Cons('b', ~Cons('c', ~Nil)));
    println!("{:?}", lastbutone(&list));

    let list: List<uint> = Cons(6, ~Cons(7, ~Cons(42, ~Nil)));
    println!("{:?}", lastbutone(&list));

    let list: List<~str> = Cons(~"six", ~Cons(~"seven", ~Cons(~"forty-two", ~Nil)));
    println!("{:?}", lastbutone(&list));
}
