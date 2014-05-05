// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 27: Vectors: groups
//!
//! Write a function that generates all the possibilities of grouping several
//! elements in several groups of specified size.
//!
//! Your function could have this signature:
//! `fn group<T: Clone>(vec: &[T], sizes: &[uint]) -> ~[~[~[T]]]`

fn group<T: Clone>(vec: &[T], sizes: &[uint]) -> ~[~[~[T]]] {
    fn aux_one<T: Clone>(vec: &[T], k: uint) -> Vec<(Vec<T>, Vec<T>)> {
        match (vec.len(), k) {
            (_, 0) => vec!((Vec::new(), Vec::from_slice(vec))),
            (l, _) if l < k => Vec::new(),
            (l, _) if l == k => vec!((Vec::from_slice(vec), Vec::new())),
            _ => {
                let mut r = Vec::new();
                for (i, elem) in vec.iter().enumerate() {
                    let s = aux_one(vec.slice_to(i), k-1);
                    r.push_all_move(
                        s.move_iter()
                        .map(
                            |(v, r)| (v.append_one(elem.clone()), r.append(vec.slice_from(i+1)))
                            ).collect()
                        )
                }
                r
            }
        }
    }
    fn aux<T: Clone>(vec: &[T], sizes: &[uint]) -> Vec<Vec<Vec<T>>> {
        match sizes {
            [] => vec!(Vec::new()),
            [k] => aux_one(vec, k).move_iter().map(|(v, _)| vec!(v)).collect(),
            [..rest, e] => {
                let mut r = Vec::new();
                for (v, new_vec) in aux_one(vec, e).move_iter() {
                    r.push_all_move(
                        aux(new_vec.as_slice(), rest).move_iter()
                        .map(|tail| tail.append_one(v.clone())).collect())
                }
                r
            }
        }
    }

    aux(vec, sizes).move_iter()
        .map(|v| v.move_iter().map(|w| w.as_slice().to_owned())
             .collect::<Vec<~[T]>>().as_slice().to_owned())
        .collect::<Vec<~[~[T]]>>().as_slice().to_owned()
}

#[test]
fn multinomial_3_5_7() {
    let vec = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
               'j', 'k', 'l', 'm', 'n', 'o'];
    assert_eq!(group(vec, [3, 5, 7]).len(), 360360);
}

#[test]
fn multinomial_7_11() {
    let vec = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
               'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r'];
    assert_eq!(group(vec, [7, 11]).len(), 31824);
}

#[test]
fn multinomial_2_3_2_1_3() {
    let vec = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
               'j', 'k'];
    assert_eq!(group(vec, [2, 3, 2, 1, 3]).len(), 277200);
}
