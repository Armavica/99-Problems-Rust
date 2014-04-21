// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 09: Vectors: pack
//! 
//! Pack consecutive duplicates of vectors elements into sub-vectors.
//!
//! Your function could have this signature:
//! `fn pack<T: Eq>(vec: ~[T]) -> ~[~[T]]`
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

#[test]
fn pack_test() {
    let vec = ~['a', 'a', 'a', 'a',
                'b',
                'c', 'c',
                'a', 'a',
                'd',
                'e', 'e', 'e', 'e'];

    assert_eq!(pack(vec), ~[
                            ~['a', 'a', 'a', 'a'],
                            ~['b'],
                            ~['c', 'c'],
                            ~['a', 'a'],
                            ~['d'],
                            ~['e', 'e', 'e', 'e']
                         ]);
}
