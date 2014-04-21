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

#[test]
fn test09_pack_() {
    let vector = ~['a', 'a', 'a', 'a', 'b', 'c', 'c', 'a',
    'a', 'd', 'e', 'e', 'e', 'e'];

    assert!(pack(vector) == ~[
            ~['a', 'a', 'a', 'a'],
            ~['b'],
            ~['c', 'c'],
            ~['a', 'a'],
            ~['d'],
            ~['e', 'e', 'e', 'e']
            ]);
}
