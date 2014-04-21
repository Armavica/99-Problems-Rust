// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 10: Vectors: encode
//!
//! Run-length encoding of a vector.
//! Memory refresh: http://en.wikipedia.org/wiki/Run-length_encoding
//!
//! Your function could have this signature:
//! `fn encode<T>(vec: ~[~[T]]) -> ~[(uint, T)]`
//!

#[deriving(Eq)]
enum Node<T> {
    One(T),
    Many(uint, T)
}

fn pack<T: Clone+Eq>(vector: &[T]) -> ~[~[T]] {
    let mut it = vector.iter();
    let mut result = ~[];
    let mut l = 1;
    loop {
        match it.nth(l - 1) {
            Some(e) => {
                let mut slice = ~[];
                slice.push(e.clone());
                for f in it.take_while(|&a| *a == *e) {
                    slice.push(f.clone());
                }
                l = slice.len();
                result.push(slice);
            },
            None    => break
        }
    }
    result
}

fn encode<T: Clone>(vector: ~[~[T]]) -> ~[Node<T>] {
    vector.map(|e| match e.len() {
        1 => One(e[0].clone()),
        n => Many(n, e[0].clone())
    })
}

#[test]
fn test10_encode() {
    let vector = ~['a', 'a', 'a', 'a', 'b', 'c', 'c',
    'a', 'a', 'd', 'e', 'e', 'e', 'e'];

    assert!(encode(pack(vector)) == ~[Many(4, 'a'),
    One('b'),
    Many(2, 'c'),
    Many(2, 'a'),
    One('d'),
    Many(4, 'e')])
}
