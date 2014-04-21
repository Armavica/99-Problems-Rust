// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 07: Vectors: flatten
//!
//! A way to define a nested vector in Rust is to think of it as a vector of
//! nodes, a node being defined by the following type:
//! ```
//! enum Node<T> {
//!     One(T),
//!     Many(~[Node<T>])
//! }
//! ```
//!
//! Flatten a nested vector structure.
//!
//! Your function could have this signature:
//! `fn flatten<T>(n_vector: Node<T>) -> ~[T]`
//!

enum Node<T> {
    One(T),
    Many(~[Node<T>])
}

fn flatten<T>(nestvec: Node<T>) -> ~[T] {
    match nestvec {
        One(v)      => ~[v],
        Many(vec)   =>
            vec.move_iter().flat_map(|node| flatten(node).move_iter()).collect()
    }
}

#[test]
fn flatten_test() {
    let nestvec =
        Many(~[One('a'), Many(~[One('b'), Many(~[One('c'), One('d')]), One('e')])]);
    assert_eq!(flatten(nestvec), ~['a', 'b', 'c', 'd', 'e']);
}

