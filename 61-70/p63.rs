#[deriving(Eq)]
enum BinaryTree<T> {
    Node(T, ~BinaryTree<T>, ~BinaryTree<T>),
    Empty
}

fn complete_binary_tree(n: uint) -> BinaryTree<uint> {
    fn aux(start: uint, n: uint) -> BinaryTree<uint> {
        if start > n {
            Empty
        } else if 2*start + 1 > n {
            Node(start, ~aux(2*start, n),
                        ~Empty)
        } else {
            Node(start, ~aux(2*start, n),
                        ~aux(2*start+1, n))
        }
    }
    aux(1, n)
}

fn main() {
    assert!(complete_binary_tree(6) == Node(1, ~Node(2, ~Node(4, ~Empty, ~Empty),
                                                        ~Node(5, ~Empty, ~Empty)),
                                               ~Node(3, ~Node(6, ~Empty, ~Empty),
                                                        ~Empty)));
}

