enum NestedList<T> {
    Nil,
    One(T, ~NestedList<T>),
    Many(~NestedList<T>, ~NestedList<T>)
}

fn rev<T>(list: NestedList<T>) -> NestedList<T> {
    fn rev_aux<T>(list: NestedList<T>, acc: NestedList<T>) -> NestedList<T> {
        match list {
            Nil => acc,
            One(elem, ~rest) => rev_aux(rest, One(elem, ~acc)),
            Many(~node, ~rest) => rev_aux(rest, Many(~rev(node), ~acc))
        }
    }
    rev_aux(list, Nil)
}

fn flatten<T: Clone>(list: &NestedList<T>) -> NestedList<T> {
    fn aux<T: Clone>(list: &NestedList<T>, acc: NestedList<T>) -> NestedList<T> {
        match *list {
            Nil => acc,
            One(ref elem, ~ref rest) => aux(rest, One(elem.clone(), ~acc)),
            Many(~ref node, ~ref rest) => aux(rest, aux(node, acc))
        }
    }

    rev(aux(list, Nil))
}


fn main() {
    let list: NestedList<char> =
        One('a', ~Many(
                ~One('b', ~Many(
                        ~One('c', ~One('d', ~Nil)),
                        ~One('e', ~Nil)
                    )),
            ~Nil));

    println!("{:?}", flatten(&list));
}
