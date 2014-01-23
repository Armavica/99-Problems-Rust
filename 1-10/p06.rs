enum List<T> {
    Cons(T, ~List<T>),
    Nil
}

impl<T: Eq> Eq for List<T> {
    fn eq(&self, ys: &List<T>) -> bool {
        match (self, ys) {
            (&Nil, &Nil) => true,
            (&Cons(ref x, ~ref rest_x), &Cons(ref y, ~ref rest_y)) if x == y =>
                rest_x == rest_y,
            _ => false
        }
    }
}

fn is_palindrome<T: Clone+Eq>(list: &List<T>) -> bool {
    fn rev<T: Clone>(list: &List<T>) -> List<T> {
        fn rev_aux<T: Clone>(list: &List<T>, acc: List<T>) -> List<T> {
            match *list {
                Nil => acc,
                Cons(ref elem, ~ref rest) => rev_aux(rest, Cons((*elem).clone(), ~acc))
            }
        }
        rev_aux(list, Nil)
    }
    list == &rev(list)
}

fn main() {
    let kayak: List<char> =
        Cons('k', ~Cons('a', ~Cons('y', ~Cons('a', ~Cons('k', ~Nil)))));
    let list: List<char> =
        Cons('l', ~Cons('i', ~Cons('s', ~Cons('t', ~Nil))));

    assert!(is_palindrome(&kayak));
    assert!(!is_palindrome(&list));
}
