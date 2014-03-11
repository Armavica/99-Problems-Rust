enum BinaryTree<T> {
    Node(T, ~BinaryTree<T>, ~BinaryTree<T>),
    Empty
}

fn leaves<T>(tree: BinaryTree<T>) -> ~[T] {
    match tree {
        Empty                   => ~[],
        Node(x, ~Empty, ~Empty) => ~[x],
        Node(_, ~left, ~right)  =>  leaves(left).move_iter()
                                    .chain(leaves(right).move_iter())
                                    .to_owned_vec()
    }
}


fn main() {
    let t1: BinaryTree<uint> = Empty;
    let t2 = Node('x', ~Node('y', ~Empty, ~Empty), ~Node('z', ~Node('t', ~Empty, ~Empty), ~Empty));
    assert_eq!(leaves(t1), ~[]);
    assert_eq!(leaves(t2), ~['y', 't']);
}

