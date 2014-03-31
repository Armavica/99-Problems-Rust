//! Problem 08: Vectors: compress
//!
//! Eliminate consecutive duplicates of vector elements.
//!
//! Your function must have this signature:
//! `fn compress<T: Eq>(vector: ~[T]) -> ~[T]`
//!

fn compress<T: Eq>(vector: ~[T]) -> ~[T] {
    let mut vector = vector;
    vector.dedup();
    vector
}

#[test]
fn test08_compress() {
    let vector = ~['a', 'a', 'a', 'a', 'b', 'c', 'c',
    'a', 'a', 'd', 'e', 'e', 'e', 'e'];

    assert!(compress(vector) == ~['a', 'b', 'c', 'a', 'd', 'e']);
}

