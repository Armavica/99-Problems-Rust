// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

extern crate collections;
extern crate rand;
use collections::hashmap::HashMap;
use std::cmp::min;
use rand::{task_rng, Rng};


/// Problem 01: Vectors: last
///
/// Write a function that returns a reference to the last element of a vector.
/// Your function could have this signature:
/// `fn last<'a, T>(vec: &'a [T]) -> Option<&'a T>`
///

fn last<'a, T>(vec: &'a [T]) -> Option<&'a T> {
    match vec {
        [] => None,
        [.., ref x] => Some(x)
    }
}

#[test]
fn last_nonempty() {
    let vec = ~['a', 'b', 'c'];
    assert_eq!(last(vec), Some(&'c'));
}

#[test]
fn last_empty() {
    let vec: ~[char] = ~[];
    assert_eq!(last(vec), None);
}

/// Problem 02: Vectors: last but one
///
/// Find the last but one (last and penultimate) elements of a vector.
/// Your function could have this signature:
/// `fn last_but_one<'a, T>(vec: &'a [T]) -> Option<(&'a T, &'a T)>`
///

fn last_but_one<'a, T>(vec: &'a [T]) -> Option<(&'a T, &'a T)> {
    match vec {
        [] | [_] => None,
        [.., ref x, ref y] => Some((x, y))
    }
}

#[test]
fn lastbutone_nonempty() {
    let vec = ~['a', 'b', 'c', 'd', 'e'];
    assert_eq!(last_but_one(vec), Some((&'d', &'e')));
}

#[test]
fn lastbutone_almostempty() {
    let vec = ~['a'];
    assert_eq!(last_but_one(vec), None);
}

#[test]
fn lastbutone_empty() {
    let vec: ~[char] = ~[];
    assert_eq!(last_but_one(vec), None);
}

/// Problem 03: Vectors: kth
///
/// Find the `k`th element of a vector.
/// Your function could have this signature:
/// `fn kth<'a, T>(k: uint, vec: &'a [T]) -> Option<&'a T>`
///

fn kth<'a, T>(k: uint, vec: &'a [T]) -> Option<&'a T> {
    if vec.len() >= k && k > 0{
        Some(&vec[k-1])
    } else {
        None
    }
}

#[test]
fn kth_outofrange() {
    let vec = ~['a'];
    assert_eq!(kth(3, vec), None)
}

#[test]
fn kth_inrange() {
    let vec = ~['a', 'b', 'c', 'd', 'e'];
    assert_eq!(kth(3, vec), Some(&'c'));
}

/// Problem 04: Vectors: length
///
/// Compute the number of elements of a vector.
///
/// Your function could have this signature:
/// `fn length<T>(vec: &[T]) -> uint`
///

fn length<T>(vec: &[T]) -> uint {
    vec.len()
}

#[test]
fn length_nonempty() {
    let vec = ~['a', 'b', 'c'];
    assert_eq!(length(vec), 3);
}

#[test]
fn length_empty() {
    let vec: ~[uint] = ~[];
    assert_eq!(length(vec), 0);
}

/// Problem 05: Vectors: reverse
///
/// Reverse a vector.
///
/// Your function could have this signature:
/// `fn reverse<T>(vec: ~[T]) -> ~[T]`
///

fn reverse<T>(vec: ~[T]) -> ~[T] {
    vec.move_iter().rev().collect()
}

#[test]
fn reverse_nonempty() {
    let vec = ~['a', 'b', 'c', 'd', 'e'];
    assert_eq!(reverse(vec), ~['e', 'd', 'c', 'b', 'a']);
}

#[test]
fn reverse_empty() {
    let vec: ~[char] = ~[];
    assert_eq!(reverse(vec), ~[]);
}

/// Problem 06: Vectors: is_palindrome
///
/// Determine if the vector passed is a palindrome or not.
///
/// Your function could have this signature:
/// `fn is_palindrome<T: Eq>(vector: &[T]) -> bool`

/// Returns whether the argument is a palindrome, recursive version
fn is_palindrome_rec<T: Eq>(vector: &[T]) -> bool {
    match vector {
        [] | [_] => true,
        [ref x, ..rest, ref y] if x == y => is_palindrome_rec(rest),
        _ => false
    }
}

/// Returns whether the argument is a palindrome, version with iterators
fn is_palindrome_it<T: Eq>(vector: &[T]) -> bool {
    vector.iter().zip(vector.iter().rev()).all(|(a, b)| a == b)
}

#[test]
fn odd_palindrome() {
    let vector = ~['r', 'a', 'c', 'e', 'c', 'a', 'r'];
    assert!(is_palindrome_rec(vector));
    assert!(is_palindrome_it(vector));
}

#[test]
fn even_palindrome() {
    let vector = ~['n', 'o', 'o', 'n'];
    assert!(is_palindrome_rec(vector));
    assert!(is_palindrome_it(vector));
}

#[test]
fn not_palidrome() {
    let vector = ~['a', 'b', 'c', 'b', 'e'];
    assert!(!is_palindrome_rec(vector));
    assert!(!is_palindrome_it(vector));
}

/// Problem 07: Vectors: flatten
///
/// A way to define a nested vector in Rust is to think of it as a vector of
/// nodes, a node being defined by the following type:
/// ```
/// enum Node<T> {
///     One(T),
///     Many(~[Node<T>])
/// }
/// ```
///
/// Flatten a nested vector structure.
///
/// Your function could have this signature:
/// `fn flatten<T>(n_vector: Node<T>) -> ~[T]`
///

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

/// Problem 08: Vectors: compress
///
/// Eliminate consecutive duplicates of vector elements.
///
/// Your function could have this signature:
/// `fn compress<T: Eq>(vec: ~[T]) -> ~[T]`
///

fn compress_rec<T: Clone+Eq>(vec: ~[T]) -> ~[T] {
    let mut result = Vec::new();
    for elem in vec.move_iter() {
        if result.last().is_none() || result.last().unwrap() != &elem {
            result.push(elem)
        }
    }
    result.as_slice().to_owned()
}

fn compress_lib<T: Clone+Eq>(vec: ~[T]) -> ~[T] {
    let mut r: Vec<T> = vec.move_iter().collect();
    r.dedup();
    r.as_slice().to_owned()
}

#[test]
fn compress_test() {
    let vec = ~['a', 'a', 'a', 'a', 'b', 'c', 'c',
                'a', 'a', 'd', 'e', 'e', 'e', 'e'];

    assert_eq!(compress_rec(vec.clone()), ~['a', 'b', 'c', 'a', 'd', 'e']);
    assert_eq!(compress_lib(vec), ~['a', 'b', 'c', 'a', 'd', 'e']);
}

/// Problem 09: Vectors: pack
///
/// Pack consecutive duplicates of vectors elements into sub-vectors.
///
/// Your function could have this signature:
/// `fn pack<T: Eq>(vec: ~[T]) -> ~[~[T]]`
///

fn pack<T: Clone+Eq>(vec: ~[T]) -> ~[~[T]] {
    let mut result: Vec<Vec<T>> = Vec::new();

    for elem in vec.move_iter() {
        if result.last().is_none() || result.last().unwrap().last().unwrap() != &elem {
            result.push(vec!(elem))
        } else {
            result.mut_last().unwrap().push(elem)
        }
    }
    result.move_iter().map(|v| v.as_slice().to_owned()).collect::<Vec<~[T]>>().as_slice().to_owned()
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
/// Problem 10: Vectors: encode
///
/// Run-length encoding of a vector.
/// Memory refresh: http://en.wikipedia.org/wiki/Run-length_encoding
///
/// Your function could have this signature:
/// `fn encode<T>(vec: ~[~[T]]) -> ~[(uint, T)]`
///

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

/// Problem 11: Vectors: encode_bis
///
/// Run-length encoding of a vector, such that when an element is alone, it
/// should be encoded separately.
/// Your output could be a list of Elem elements, Elem being defined as:
/// ```
/// enum Elem<T> {
///     Unique(T),
///     Several(uint, T)
/// }
/// ```
///
/// Your function could have this signature:
/// `fn encode_bis<T>(vec: ~[~[T]]) -> ~[Elem<T>]`
///

#[deriving(Eq)]
#[deriving(Clone)]
#[deriving(Show)]
enum Elem<T> {
    Unique(T),
    Several(uint, T)
}


fn encode_bis<T>(vec: ~[~[T]]) -> ~[Elem<T>] {
    vec.move_iter().map(|e|
                        match e.len() {
                            0 => unreachable!(),
                            1 => Unique(e[0]),
                            n => Several(n, e[0])
                        }).collect()
}

#[test]
fn encode_bis_test() {
    let vec = ~['a', 'a', 'a', 'a', 'b', 'c', 'c',
                'a', 'a', 'd', 'e', 'e', 'e', 'e'];

    assert_eq!(encode_bis(pack(vec)), ~[Several(4, 'a'),
                                        Unique('b'),
                                        Several(2, 'c'),
                                        Several(2, 'a'),
                                        Unique('d'),
                                        Several(4, 'e')])
}

/// Problem 12: Vectors: decode
///
/// Given a run-length encoded vector generated by the function of the
/// previous problem, construct his uncompressed version.
///
/// Your function could have this signature:
/// `fn decode<T: Clone>(vec: ~[Elem<T>]) -> ~[T]`

fn decode<T: Clone>(list: ~[Elem<T>]) -> ~[T] {
    list.move_iter().flat_map(|e|
                match e {
                    Unique(a)     => {
                        Vec::from_elem(1, a).move_iter()
                    }
                    Several(n, a) => {
                        Vec::from_elem(n, a).move_iter()
                    }
                }).collect()
}

#[test]
fn decode_test() {
    let vec = ~[Several(4, 'a'),
                Unique('b'),
                Several(2, 'c'),
                Several(2, 'a'),
                Unique('d'),
                Several(4, 'e')];
    assert_eq!(decode(vec), ~[  'a', 'a', 'a', 'a',
                                'b',
                                'c', 'c',
                                'a', 'a',
                                'd',
                                'e', 'e', 'e', 'e']);
}

/// Problem 13: Vectors: encode_ter
///
/// Implement the so-called run-length encoding data compression method
/// directly. I.e. don't explicitly create the sublists containing the
/// duplicates, as in problem "Pack consecutive duplicates of list elements
/// into sublists", but only count them. As in problem "Modified run-length
/// encoding", simplify the result list by replacing the singleton lists
/// (1 X) by X.
///
/// Your function could have this signature:
/// `fn encode_ter<T>(vec: ~[T]) -> ~[Elem<T>]`


fn encode_ter<T: Clone+Eq>(vec: ~[T]) -> ~[Elem<T>] {
    let mut input = vec.move_iter().peekable();
    let mut n = 1;
    let mut result = Vec::new();

    while !input.peek().is_none() {
        match (input.next(), input.peek()) {
            (Some(a), Some(b)) =>
                if a != *b {
                    if n == 1 {
                        result.push(Unique(a));
                    } else {
                        result.push(Several(n, a));
                        n = 1;
                    }
                } else {
                    n += 1
                },
            (Some(a), None) => {result.push(Several(n, a)); n = 1},
            (None, _) => unreachable!()
        }
    }
    result.as_slice().to_owned()
}

#[test]
fn encode_ter_test() {
    let vec = ~['a', 'a', 'a', 'a',
                'b',
                'c', 'c',
                'a', 'a',
                'd',
                'e', 'e', 'e', 'e'];

    assert_eq!(encode_ter(vec), ~[  Several(4, 'a'),
                                    Unique('b'),
                                    Several(2, 'c'),
                                    Several(2, 'a'),
                                    Unique('d'),
                                    Several(4, 'e')])
}

/// Problem 14: Vectors: duplicate
///
/// Duplicate the elements of a vector.
///
/// Your function could have this signature:
/// `fn dupli<T: Clone>(vec: ~[T]) -> ~[T]`

fn dupli<T: Clone>(vec: ~[T]) -> ~[T] {
    vec.move_iter().flat_map(|e| Vec::from_elem(2, e).move_iter()).collect()
}

#[test]
fn dupli_test() {
    let vec = ~['a', 'b', 'c', 'c', 'd'];
    assert_eq!(dupli(vec), ~['a', 'a', 'b', 'b', 'c', 'c', 'c', 'c', 'd', 'd']);
}

/// Problem 15: Vectors: replicate
///
/// Replicate the elements of a vector a given number of times.
///
/// Your function could have this signature:
/// `fn repli<T: Clone>(n: uint, vec: ~[T]) -> ~[T]`

fn repli<T: Clone>(n: uint, vec: ~[T]) -> ~[T] {
    vec.move_iter().flat_map(|e| Vec::from_elem(n, e).move_iter()).collect()
}

#[test]
fn dupli_0() {
    let vec = ~['a', 'b', 'c', 'c', 'd'];
    assert_eq!(repli(0, vec), ~[]);
}

#[test]
fn dupli_2() {
    let vec = ~['a', 'b', 'c', 'c', 'd'];
    assert_eq!(repli(2, vec), ~['a', 'a', 'b', 'b', 'c', 'c', 'c', 'c', 'd', 'd']);
}

/// Problem 16: Vectors: drop
///
/// Drop every `n`th element of a vector.
///
/// Your function could have this signature:
/// `fn drop<T>(n: uint, vec: ~[T]) -> ~[T]`

fn drop<T>(vec: ~[T], n: uint) -> ~[T] {
    vec.move_iter().enumerate().filter_map(
        |(i,e)|
            if i%n < n-1 {
                Some(e)
            } else {
                None
            }
        ).collect()
}

#[test]
fn drop_test() {
    let vec = ~['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
    assert_eq!(drop(vec, 3), ~['a', 'b', 'd', 'e', 'g', 'h', 'j']);
}


/// Problem 17: Vectors: split
///
/// Split a vector into two parts; the length of the first part is given.
/// If the length of the first part is longer than the entire vec, then the
/// first part is the vec and the second part is empty.
///
/// Your function could have this signature:
/// `fn split<T>(vec: &[T], n: uint) -> (&[T], &[T])`

fn split<'a, T>(vec: &'a [T], n: uint) -> (&'a [T], &'a [T]) {
    (vec.slice_to(min(n, vec.len())), vec.slice_from(min(n, vec.len())))
}

#[test]
fn split_ok() {
    let vec = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
    assert_eq!(split(vec, 3),
        (&['a', 'b', 'c'], &['d', 'e', 'f', 'g', 'h', 'i', 'j']));
}

#[test]
fn split_begin() {
    let vec = ['a', 'b', 'c', 'd', 'e'];
    assert_eq!(split(vec, 0),
        (&[], &['a', 'b', 'c', 'd', 'e']));
}

#[test]
fn split_end() {
    let vec = ['a', 'b', 'c', 'd', 'e'];
    assert_eq!(split(vec, 5),
        (&['a', 'b', 'c', 'd', 'e'], &[]));
}

#[test]
fn split_longer() {
    let vec = ['a', 'b', 'c', 'd', 'e'];
    assert_eq!(split(vec, 10),
        (&['a', 'b', 'c', 'd', 'e'], &[]));
}

/// Problem 18: Vectors: slice
///
/// Given two indices, `i` and `k`, the slice is the vector containing the
/// elements between the `i`th and `k`th element of the original vector (first
/// limit included only).  Start counting the elements with 0.  Your function
/// should not fail in case one or the other indices point outside the vector.
///
/// Your function could have this signature:
/// `fn dupli<T>(vec: &'a [T], i: uint, k: uint) -> &'a [T]`

fn slice<'a, T>(vec: &'a [T], i: uint, k: uint) -> &'a [T] {
    vec.slice(min(i, vec.len()), min(k, vec.len()))
}

#[test]
fn slice_ok() {
    let vec = ~['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
    assert_eq!(slice(vec, 3, 7), &['d', 'e', 'f', 'g']);
}

#[test]
fn slice_outside() {
    let vec = ~['a', 'b', 'c', 'd', 'e'];
    assert_eq!(slice(vec, 7, 10), &[]);
}


/// Problem 19: Vectors: rotate
///
/// Rotate a vector N places to the left.
///
/// Your function could have this signature:
/// `fn rotate<T>(vec: ~[T], n: int) -> ~[T]`

fn rotate<T: Clone>(vec: ~[T], n: int) -> ~[T] {
    let mut res = Vec::new();
    let l = vec.len() as int;
    let r: uint = (((n % l) + l) % l) as uint;

    res.push_all(vec.slice_from(r));
    res.push_all(vec.slice_to(r));
    res.as_slice().to_owned()
}

#[test]
fn rotate_left() {
    let vec = ~['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    assert_eq!(rotate(vec, 3), ~['d', 'e', 'f', 'g', 'h', 'a', 'b', 'c']);
}

#[test]
fn rotate_right() {
    let vec = ~['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    assert_eq!(rotate(vec, -2), ~['g', 'h', 'a', 'b', 'c', 'd', 'e', 'f']);
}

#[test]
fn rotate_mod() {
    let vec = ~['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    assert_eq!(rotate(vec, -10), ~['g', 'h', 'a', 'b', 'c', 'd', 'e', 'f']);
}


/// Problem 20: Vectors: remove_at
///
/// Return a vector with its `k`th element removed.
///
/// Your function could have this signature:
/// `fn remove_at<T>(vec: ~[T], k: uint) -> ~[T]`

fn remove_at<T>(vec: ~[T], k: uint) -> ~[T] {
    vec.move_iter()
       .enumerate()
       .filter_map(|(n, e)| if n != k {Some(e)} else {None})
       .collect()
}

#[test]
fn remove_at_in() {
    let list = ~['a', 'b', 'c', 'd'];
    assert_eq!(remove_at(list, 2), ~['a', 'b', 'd']);
}

#[test]
fn remove_at_out() {
    let list = ~['a', 'b', 'c', 'd'];
    assert_eq!(remove_at(list, 10), ~['a', 'b', 'c', 'd']);
}

/// Problem 21: Vectors: insert
///
/// Insert an element at a given position into a vector.  Start counting vector
/// elements with 0. If the position is larger or equal to the length of the
/// vector, insert the element at the end.
///
/// Your function could have this signature:
/// `fn insert<T>(elem: T, vec: Vec<T>, k: uint) -> Vec<T>`

fn insert<T>(elem: T, vec: Vec<T>, k: uint) -> Vec<T> {
    let mut vec = vec;
    let l = vec.len();
    vec.insert(min(k, l), elem);
    vec
}

#[test]
fn insert_in() {
    let vec = vec!('a', 'b', 'c', 'd');
    assert_eq!(insert('X', vec, 2), vec!('a', 'b', 'X', 'c', 'd'));
}

#[test]
fn insert_out() {
    let vec = vec!('a', 'b', 'c', 'd');
    assert_eq!(insert('X', vec, 10), vec!('a', 'b', 'c', 'd', 'X'));
}


/// Problem 22: Vectors: range
///
/// Create a vector containing all integers within a given range.  If the first
/// argument is smaller than the second, produce a vector in decreasing order.
///
/// Your function could have this signature:
/// `fn rotate(from: int, to: int) -> ~[int]`

fn rang(from: int, to: int) -> ~[int] {
    if to >= from {
        range(from, to).collect()
    } else {
        range(to+1, from+1).rev().collect()
    }
}

#[test]
fn range_increasing() {
    assert_eq!(rang(4, 9), ~[4, 5, 6, 7, 8]);
}

#[test]
fn range_decreasing() {
    assert_eq!(rang(9, 4), ~[9, 8, 7, 6, 5]);
}


/// Problem 23: Vectors: rnd_select
///
/// Extract a given number of randomly selected elements from a vector.
/// The selected items shall be returned in a vector.
///
/// Your function could have this signature:
/// `fn rnd_select<'a, T>(vec: &'a [T], n: uint) -> Vec<&'a T>`


fn rnd_select<'a, T>(vec: &'a [T], n: uint) -> Vec<&'a T> {
    task_rng().sample(vec.iter(), n)
}

/// Problem 24: Vectors: lotto_select
///
/// Lotto: Draw `k` different random numbers from the set `0..n-1`.
///
/// Your function could have this signature:
/// `fn lotto_select(k: uint, n: uint) -> Vec<uint>`


fn lotto_select(k: uint, n: uint) -> Vec<uint> {
    task_rng().sample(range(0, n), k)
}

/// Problem 25: Vectors: rnd_permu
///
/// Generate a random permutation of the elements of a vector.
///
/// Your function could have this signature:
/// `fn rnd_permu<T>(vec: ~[T]) -> Vec<T>`


fn rnd_permu<T>(vec: ~[T]) -> ~[T] {
    let mut vec = vec;
    task_rng().shuffle(vec);
    vec
}

/// Problem 26: Vectors: combinations
///
/// Generate the combinations of `k` distinct objects chosen from all the
/// elements of a vector.
///
/// Your function could have this signature:
/// `fn combinations<T: Clone>(vec: &[T], k: uint) -> ~[~[T]]`

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

/// Problem 27: Vectors: groups
///
/// Write a function that generates all the possibilities of grouping several
/// elements in several groups of specified size.
///
/// Your function could have this signature:
/// `fn group<T: Clone>(vec: &[T], sizes: &[uint]) -> ~[~[~[T]]]`

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

/// Problem 28: Vectors: sorts
///
/// We suppose that a vector contains element that are vectors themselves.
/// Write a function `length_sort` which sorts the elements of this vector
/// according to their length.
///
/// Your function could have this signature:
/// `fn length_sort<T>(vec: ~[~[T]]) -> ~[~[T]]`
///
/// Now write a function `frequenry_sort` which sorts the elements of this
/// vector according to their length frequency in the vector: i.e. vector with
/// rare lengths are placed first, others with a more frequent length come
/// later.
///
/// Your function could have this signature:
/// `fn frequency_sort<T>(vec: ~[~[T]]) -> ~[~[T]]`


fn length_sort<T>(vec: ~[~[T]]) -> ~[~[T]] {
    let mut vec = vec;
    vec.sort_by(|a, b| a.len().cmp(&b.len()));
    vec
}

fn frequency_sort<T>(vec: ~[~[T]]) -> ~[~[T]] {
    let mut freqs = HashMap::<uint, uint>::new();
    for v in vec.iter() {
        freqs.insert_or_update_with(v.len(), 1, |_, n| *n += 1);
    }
    let mut vec = vec;
    vec.sort_by(|a, b| freqs.get(&a.len()).cmp(freqs.get(&b.len())));
    vec
}

#[test]
fn length_test() {
    let vec = ~[~['a', 'b', 'c'], ~['d', 'e'], ~['f', 'g', 'h'],
                ~['d', 'e'], ~['i', 'j', 'k', 'l'], ~['m', 'n'], ~['o']];
    assert_eq!(length_sort(vec),
            ~[~['o'],
              ~['d', 'e'], ~['d', 'e'], ~['m', 'n'],
              ~['a', 'b', 'c'], ~['f', 'g', 'h'],
              ~['i', 'j', 'k', 'l']])
}

#[test]
fn frequency_test() {
    let vec = ~[~['a', 'b', 'c'], ~['d', 'e'], ~['f', 'g', 'h'],
                ~['d', 'e'], ~['i', 'j', 'k', 'l'], ~['m', 'n'], ~['o']];
    assert_eq!(frequency_sort(vec),
            ~[~['i', 'j', 'k', 'l'], ~['o'],
              ~['a', 'b', 'c'], ~['f', 'g', 'h'],
              ~['d', 'e'], ~['d', 'e'], ~['m', 'n']])
}
