// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 13: Vectors: encode_ter
//!
//! Implement the so-called run-length encoding data compression method
//! directly. I.e. don't explicitly create the sublists containing the
//! duplicates, as in problem "Pack consecutive duplicates of list elements
//! into sublists", but only count them. As in problem "Modified run-length
//! encoding", simplify the result list by replacing the singleton lists
//! (1 X) by X.
//!
//! Your function could have this signature:
//! `fn encode_ter<T>(vec: ~[T]) -> ~[Node<T>]`

#[deriving(Eq)]
#[deriving(Show)]
enum Node<T> {
    One(T),
    Many(uint, T)
}

fn encode_ter<T: Eq>(vec: ~[T]) -> ~[Node<T>] {
    let mut input = vec.move_iter().peekable();
    let mut n = 1;
    let mut result = ~[];

    while !input.peek().is_none() {
        match (input.next(), input.peek()) {
            (Some(a), Some(b)) =>
                if a != *b {
                    if n == 1 {
                        result.push(One(a));
                    } else {
                        result.push(Many(n, a));
                        n = 1;
                    }
                } else {
                    n += 1
                },
            (Some(a), None) => {result.push(Many(n, a)); n = 1},
            (None, _) => unreachable!()
        }
    }
    result
}

#[test]
fn encode_ter_test() {
    let vec = ~['a', 'a', 'a', 'a',
                'b',
                'c', 'c',
                'a', 'a',
                'd',
                'e', 'e', 'e', 'e'];

    assert_eq!(encode_ter(vec), ~[  Many(4, 'a'),
                                    One('b'),
                                    Many(2, 'c'),
                                    Many(2, 'a'),
                                    One('d'),
                                    Many(4, 'e')])
}
