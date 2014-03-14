// Licence boiler plate?
//

//! Problem 01: Vectors: last
//! 
//! Write a function that returns the last element of a vector.
//! Your function must have this signature:
//! `fn last<'a, T>(vector: &'a [T]) -> Option<&'a T>`
//!

fn last<'a, T>(vector: &'a [T]) -> Option<&'a T> {
    match vector {
        [] => None,
        [.., ref x] => Some(x)
    }
}

#[test]
fn test01_last() {
    let vector = ['a', 'b', 'c'];
    let empty: &[char] = [];

    assert!(last(vector) == Some(&'c'));
    assert!(vector.last() == Some(&'c'));
    assert!(last(empty) == None);
    assert!(empty.last() == None);
}

