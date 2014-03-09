enum Node<T> {
    One(T),
    Many(~[Node<T>])
}

fn flatten<T: Clone>(nlist: &~[Node<T>]) -> ~[T] {
    nlist.flat_map(|n|
                   match *n {
                       One(ref e) => ~[e.clone()],
                       Many(ref el) => flatten(el)
                   } )
}

fn main() {
    let nlist =
        ~[One('a'), Many(~[One('b'), Many(~[One('c'), One('d')]), One('e')])];

    assert!(flatten(&nlist) == ~['a', 'b', 'c', 'd', 'e']);
}
