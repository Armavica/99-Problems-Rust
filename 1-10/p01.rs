enum List<T> {
    Cons(T, ~List<T>),
    Nil
}

fn last<'a, T>(list: &'a List<T>) -> Option<&'a T> {
    match *list {
        Nil => None,
        Cons(ref elem, ~Nil) => Some(elem),
        Cons(_, ~ref rest) => last(rest)
    }
}

fn main() {
    let list: List<char> = Cons('a', ~Cons('b', ~Cons('c', ~Nil)));
    println!("{:?}", last(&list));

    let list: List<uint> = Cons(6, ~Cons(7, ~Cons(42, ~Nil)));
    println!("{:?}", last(&list));

    let list: List<~str> = Cons(~"six", ~Cons(~"seven", ~Cons(~"forty-two", ~Nil)));
    println!("{:?}", last(&list));
}
