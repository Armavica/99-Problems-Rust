// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 26: Vectors: combinations
//!
//! Generate the combinations of `k` distinct objects chosen from all the
//! elements of a vector.
//!
//! Your function could have this signature:
//! `fn combinations<T: Clone>(vec: &[T], k: uint) -> ~[~[T]]`

fn combinations<T: Clone>(vec: &[T], k: uint) -> ~[~[T]] {
    fn aux<T: Clone>(vec: &[T], k: uint) -> Vec<Vec<T>> {
        match (vec.len(), k) {
            (_, 0) => vec!(Vec::new()),
            (l, _) if l < k => Vec::new(),
            (l, _) if l == k => vec!(Vec::from_slice(vec)),
            _ => {
                let mut r = Vec::new();
                for (i, elem) in vec.iter().enumerate() {
                    let s = aux(vec.slice_to(i), k-1);
                    r.push_all_move(
                        s.move_iter()
                        .map(
                            |v| v.append_one(elem.clone())
                            ).collect()
                        )
                }
                r
            }
        }
    }
    aux(vec, k).move_iter()
        .map(|v| v.as_slice().to_owned())
        .collect::<Vec<~[T]>>().as_slice().to_owned()
}

#[test]
fn combinations_test() {
    let vec = ['a', 'b', 'c', 'd'];
    assert_eq!(combinations(vec, 2),
        ~[~['a', 'b'],
          ~['a', 'c'],
          ~['b', 'c'],
          ~['a', 'd'],
          ~['b', 'd'],
          ~['c', 'd']])
}
