/// Returns whether the argument is a palindrome, recursive version
fn is_palindrome_pm<T: Eq>(list: &[T]) -> bool {
    match list {
        [] | [_] => true,
        [ref x, ..rest, ref y] if x == y => is_palindrome_pm(rest),
        _ => false
    }
}

/// Returns whether the argument is a palindrome, iterative version
fn is_palindrome_it<T: Eq>(list: &[T]) -> bool {
    let mut comp = list.iter().zip(list.rev_iter());
    for (a, b) in comp {
        if *a != *b {
            return false
        }
    }
    true
}

fn main() {
    let list = ~['x', 'a', 'm', 'a', 'x'];
    assert!(is_palindrome_pm(list));
    assert!(is_palindrome_it(list));

    let list = ~['a', 'b'];
    assert!(!is_palindrome_pm(list));
    assert!(!is_palindrome_it(list));
}
