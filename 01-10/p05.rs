//
//

//! Problem 05: Vectors: rev
//!
//! Reverse a vector.
//! Rust standard library have a method for this.
//! You can use it or reimplement it.
//!
//! Your function must have this signature:
//! `fn rev<T>(vector: ~[T]) -> ~[T]`
//!

fn rev<T>(vector: ~[T]) -> ~[T] {
    vector.move_rev_iter().collect()
}

#[test]
fn test05_rev() {
    let vector = ~['a', 'b', 'c', 'd', 'e'];
    assert!(rev(vector) == ~['e', 'd', 'c', 'b', 'a']);
}

