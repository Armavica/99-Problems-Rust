enum BinaryTree<T> {
    Node(T, ~BinaryTree<T>, ~BinaryTree<T>),
    Empty
}

fn internals<T>(tree: BinaryTree<T>) -> ~[T] {
    match tree {
        Empty                   => ~[],
        Node(_, ~Empty, ~Empty) => ~[],
        Node(x, ~left, ~right)  =>  (~[x]).move_iter()
                                    .chain(internals(left).move_iter())
                                    .chain(internals(right).move_iter())
                                    .to_owned_vec()
    }
}


fn main() {
    let t1: BinaryTree<uint> = Empty;
    let t2 = Node('x', ~Node('y', ~Empty, ~Empty), ~Node('z', ~Node('t', ~Empty, ~Empty), ~Empty));
    assert_eq!(internals(t1), ~[]);
    assert_eq!(internals(t2), ~['x', 'z']);
}

