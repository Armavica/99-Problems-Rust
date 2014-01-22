enum List<T> {
    Cons(T, ~List<T>),
    Nil
}

fn length<T>(list: &List<T>) -> uint {
    match *list {
        Nil => 0,
        Cons(_, ~ref rest) => length(rest)+1
    }
}

fn main() {
    let list: List<char> = Cons('a', ~Cons('b', ~Cons('c', ~Cons('d', ~Nil))));
    println!("{}", length(&list));
}
