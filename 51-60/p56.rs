#[deriving(Clone)]
enum BinaryTree<T> {
    Node(T, ~BinaryTree<T>, ~BinaryTree<T>),
    Empty
}

fn is_mirror<T>(t1: BinaryTree<T>, t2: BinaryTree<T>) -> bool {
    match (t1, t2) {
        (Empty, Empty) => true,
        (Node(_, ~t1l, ~t1r), Node(_, ~t2l, ~t2r)) => is_mirror(t1l, t2r) && is_mirror(t1r, t2l),
        _ => false
    }
}

fn is_symmetric<T>(tree: BinaryTree<T>) -> bool {
    match tree {
        Empty => true,
        Node(_, ~l, ~r) => is_mirror(l, r)
    }
}

fn main() {
    let sym    = Node(42, ~Node(21, ~Node(1, ~Empty, ~Empty), ~Empty), ~Node(10, ~Empty, ~Node(2, ~Empty, ~Empty)));
    let notsym = Node(42, ~Node(21, ~Node(1, ~Empty, ~Empty), ~Empty), ~Node(10, ~Empty, ~Empty));
    assert!(is_symmetric(sym));
    assert!(!is_symmetric(notsym));
}
