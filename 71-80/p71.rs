use std::vec_ng::Vec;

#[deriving(Eq)]
#[deriving(Show)]
enum MultiTree<'a, T> {
    Tree(T, std::vec_ng::Vec<MultiTree<'a, T>>)
}

fn ipl<T>(tree: MultiTree<T>) -> uint {
    fn aux<T>(len: uint, Tree(_, forest): MultiTree<T>) -> uint {
        forest.move_iter().fold(len, |sum, t| sum + aux(len + 1, t))
    }
    aux(0, tree)
}

#[test]
fn p71_ipl() {
    let tree = Tree('a', vec!(Tree('f', vec!(Tree('g', vec!()))),
                              Tree('c', vec!()),
                              Tree('b', vec!(Tree('d', vec!()),
                                             Tree('e', vec!())))));
    assert_eq!(ipl(tree), 9);
}

