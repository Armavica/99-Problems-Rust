//! Problem 07: Vectors: flatten
//!
//! Flatten a nested vector structure.
//!
//! Your function must have this signature:
//! `fn flatten<t: clone>(n_vector: &~[node<t>]) -> ~[t]`
//!

enum Node<T> {
    One(T),
    Many(~[Node<T>])
}

fn flatten<T: Clone>(n_vector: &~[Node<T>]) -> ~[T] {
    n_vector.flat_map(|n| match *n {
        One(ref e) => ~[e.clone()],
        Many(ref el) => flatten(el)
    })
}

#[test]
fn test07_flatten() {
    let n_vector = ~[One('a'), Many(~[One('b'), Many(~[One('c'), One('d')]), One('e')])];

    assert!(flatten(&n_vector) == ~['a', 'b', 'c', 'd', 'e']);
}

