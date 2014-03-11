enum BinaryTree<T> {
    Node(T, ~BinaryTree<T>, ~BinaryTree<T>),
    Empty
}

fn at_level<T>(tree: BinaryTree<T>, level: uint) -> ~[T] {
    match (level, tree) {
        (1, Node(x, _, _)) => ~[x],
        (n, Node(_, ~left, ~right)) => at_level(left, n-1)
                                       .move_iter()
                                       .chain(at_level(right, n-1).move_iter())
                                       .to_owned_vec(),
        _                           => ~[]
    }
}


fn main() {
    let example_tree =
    Node('a', ~Node('b', ~Node('d', ~Empty, ~Empty), ~Node('e', ~Empty, ~Empty)),
         ~Node('c', ~Empty, ~Node('f', ~Node('g', ~Empty, ~Empty), ~Empty)));
    assert_eq!(at_level(example_tree, 2), ~['b', 'c']);
}

