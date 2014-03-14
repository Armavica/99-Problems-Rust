//
//

//! Problem 04: Vectors: length
//!
//! Find the number of elements of a vector there is a
//! method in standard library feel free to use it or
//! try to reimplement it.
//!
//! Note: Rust doesn't support tail recursion
//! explicitly so becareful with a recursive one,
//! on big vector. :)
//!
//! Your function must have this signature:
//! `fn length<T>(vector: &[T]) -> uint`
//!

fn length<T>(vector: &[T]) -> uint {
    vector.len()
}

#[test]
fn test04_length() {
    let vector = ~['a', 'b', 'c'];
    assert!(length(vector) == 3);

    let vector: ~[uint] = ~[];
    assert!(length(vector) == 0);
}

