use std::str::from_char;
use std::vec_ng::Vec;

enum MultiTree<T> {
    Tree(T, Vec<MultiTree<T>>)
}

fn lispy(Tree(head, forest): MultiTree<char>) -> ~str {
    match forest.as_slice() {
        [] => from_char(head),
        _ => "(" + forest.move_iter()
                         .fold(from_char(head), |s, t| s + " " + lispy(t)) + ")"
    }
}

#[test]
fn p73_lispy_1() {
    let tree = Tree('a', vec!());
    assert_eq!(lispy(tree), ~"a");
}

#[test]
fn p73_lispy_2() {
    let tree = Tree('a', vec!(Tree('b', vec!())));
    assert_eq!(lispy(tree), ~"(a b)");
}

#[test]
fn p73_lispy_3() {
    let tree = Tree('a', vec!(Tree('b', vec!(Tree('c', vec!())))));
    assert_eq!(lispy(tree), ~"(a (b c))");
}

#[test]
fn p73_lispy_4() {
    let tree = Tree('b', vec!(Tree('d', vec!()), Tree('e', vec!())));
    assert_eq!(lispy(tree), ~"(b d e)");
}

#[test]
fn p73_lispy_5() {
    let tree = Tree('a', vec!(Tree('f', vec!(Tree('g', vec!()))),
                              Tree('c', vec!()),
                              Tree('b', vec!(Tree('d', vec!()),
                                             Tree('e', vec!())))));
    assert_eq!(lispy(tree), ~"(a (f g) c (b d e))");
}
