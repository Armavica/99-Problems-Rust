enum BinaryTree<T> {
    Node(T, ~BinaryTree<T>, ~BinaryTree<T>),
    Empty
}

fn count_leaves<T>(tree: &BinaryTree<T>) -> uint {
    match *tree {
        Empty                   => 0,
        Node(_, ~Empty, ~Empty) => 1,
        Node(_, ~ref left, ~ref right)  => count_leaves(left) + count_leaves(right)
    }
}


fn main() {
    let t1: BinaryTree<uint> = Empty;
    let t2 = Node('x', ~Node('y', ~Empty, ~Empty), ~Node('z', ~Node('t', ~Empty, ~Empty), ~Empty));
    assert_eq!(count_leaves(&t1), 0);
    assert_eq!(count_leaves(&t2), 2);
}

