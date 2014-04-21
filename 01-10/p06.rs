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
fn is_palindrome_pm<T: Eq>(vector: &[T]) -> bool {
    match vector {
        [] | [_] => true,
        [ref x, ..rest, ref y] if x == y => is_palindrome_pm(rest),
        _ => false
    }
}

/// Returns whether the argument is a palindrome, iterative version
fn is_palindrome_it<T: Eq>(vector: &[T]) -> bool {
    let mut comp = vector.iter().zip(vector.rev_iter());
    for (a, b) in comp {
        if *a != *b {
            return false
        }
    }
    true
}

#[test]
fn test06_is_palindrome() {
    let vector = ~['x', 'a', 'm', 'a', 'x'];
    assert!(is_palindrome_pm(vector));
    assert!(is_palindrome_it(vector));

    let vector = ~['a', 'b'];
    assert!(!is_palindrome_pm(vector));
    assert!(!is_palindrome_it(vector));
}

