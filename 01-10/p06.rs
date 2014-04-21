// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 06: Vectors: is_palindrome
//!
//! Determine if the vector passed is a palindrome or not.
//!
//! Your function could have this signature:
//! `fn is_palindrome<T: Eq>(vector: &[T]) -> bool`

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
    vector.iter().zip(vector.rev_iter()).all(|(a, b)| a == b)
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

