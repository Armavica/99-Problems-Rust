// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 11: Vectors: encode_bis
//!
//! Run-length encoding of a vector, such that when an element is alone, it
//! should be encoded separately.
//! Your output could be a list of Node elements, Node being defined as:
//! ```
//! enum Node<T> {
//!     One(T),
//!     Many(uint, T)
//! }
//! ```
//!
//! Your function could have this signature:
//! `fn encode_bis<T>(vec: ~[~[T]]) -> ~[Node<T>]`
//!

#[deriving(Eq)]
#[deriving(Show)]
enum Node<T> {
    One(T),
    Many(uint, T)
}

fn pack<T: Eq>(vec: ~[T]) -> ~[~[T]] {
    let mut result: ~[~[T]] = ~[];

    for elem in vec.move_iter() {
        if result.last().is_none() || result.last().unwrap().last().unwrap() != &elem {
            result.push(~[elem])
        } else {
            result.mut_last().unwrap().push(elem)
        }
    }
    result
}

fn encode_bis<T>(vec: ~[~[T]]) -> ~[Node<T>] {
    vec.move_iter().map(|e|
                        match e.len() {
                            0 => unreachable!(),
                            1 => One(e[0]),
                            n => Many(n, e[0])
                        }).collect()
}

#[test]
fn encode_test() {
    let vec = ~['a', 'a', 'a', 'a', 'b', 'c', 'c',
                'a', 'a', 'd', 'e', 'e', 'e', 'e'];

    assert_eq!(encode_bis(pack(vec)), ~[Many(4, 'a'),
                                        One('b'),
                                        Many(2, 'c'),
                                        Many(2, 'a'),
                                        One('d'),
                                        Many(4, 'e')])
}
