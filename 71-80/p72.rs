use std::vec_ng::Vec;

#[deriving(Show)]
enum MultiTree<T> {
    Tree(T, Vec<MultiTree<T>>)
}

fn bottom_up<T>(Tree(head, forest): MultiTree<T>) -> Vec<T> {
    std::vec_ng::append_one(
        forest.move_iter().flat_map(|t| bottom_up(t).move_iter()).collect(),
        head)
}

#[test]
fn p72_bottom_up_1() {
    let tree = Tree('a', vec!(Tree('b', vec!())));
    assert_eq!(bottom_up(tree), vec!('b', 'a'));
}

#[test]
fn p72_bottom_up_2() {
    let tree = Tree('a', vec!(Tree('f', vec!(Tree('g', vec!()))),
                              Tree('c', vec!()),
                              Tree('b', vec!(Tree('d', vec!()),
                                             Tree('e', vec!())))));
    assert_eq!(bottom_up(tree), vec!('g', 'f', 'c', 'd', 'e', 'b', 'a'));
}

