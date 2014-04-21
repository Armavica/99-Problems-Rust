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

fn encode<T>(vec: ~[~[T]]) -> ~[(uint, T)] {
    vec.move_iter().map(|e| (e.len(), e[0])).collect()
}

#[test]
fn encode_test() {
    let vec = ~['a', 'a', 'a', 'a', 'b', 'c', 'c',
                'a', 'a', 'd', 'e', 'e', 'e', 'e'];

    assert_eq!(encode(pack(vec)), ~[(4, 'a'),
                                    (1, 'b'),
                                    (2, 'c'),
                                    (2, 'a'),
                                    (1, 'd'),
                                    (4, 'e')])
}
