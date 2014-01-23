enum List<T> {
    Cons(T, ~List<T>),
    Nil
}

fn rev<T: Clone>(list: &List<T>) -> List<T> {
    fn rev_aux<T: Clone>(list: &List<T>, acc: List<T>) -> List<T> {
        match *list {
            Nil => acc,
            Cons(ref elem, ~ref rest) => rev_aux(rest, Cons((*elem).clone(), ~acc))
        }
    }
    rev_aux(list, Nil)
}

fn main() {
    let list: List<char> = Cons('a', ~Cons('b', ~Cons('c', ~Cons('d', ~Nil))));
    println!("{:?}", rev(&list));
}
