//
//

//! Problem 02: Vectors: last but one
//!
//! Find the last but one (last and penultimate) elements of a vector.
//! Your function must have this signature:
//! `fn last_but_one<'a, T>(vector: &'a [T]) -> Option<(&'a T, &'a T)>`
//!

fn last_but_one<'a, T>(vector: &'a [T]) -> Option<(&'a T, &'a T)> {
    match vector {
        [] | [_] => None,
        [.., ref x, ref y] => Some((x, y))
    }
}

#[test]
fn test02_last_but_one() {
    let vector = ['a', 'b', 'c', 'd', 'e'];
    assert!(last_but_one(vector) == Some((&'d', &'e')));

    let vector = ['a'];
    assert!(last_but_one(vector) == None);
}

