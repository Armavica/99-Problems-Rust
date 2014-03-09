#[deriving(Clone)]
enum BinaryTree<T> {
    Node(T, ~BinaryTree<T>, ~BinaryTree<T>),
    Empty
}

impl<T: Eq> Eq for BinaryTree<T> {
    fn eq(&self, other: &BinaryTree<T>) -> bool {
        match (self, other) {
            (&Empty, &Empty) => true,
            (&Node(ref a, ref ll, ref lr), &Node(ref b, ref rl, ref rr)) if a == b =>
                ll == rl && lr == rr,
            _ => false
        }
    }
}

fn construct<T: Ord>(list: ~[T]) -> BinaryTree<T> {
    fn insert<T: Ord>(tree: BinaryTree<T>, elem: T) -> BinaryTree<T> {
        match tree {
            Empty           =>  Node(elem, ~Empty, ~Empty),
            Node(e, ~l, ~r) =>  if elem < e {
                                    Node(e, ~insert(l, elem), ~r)
                                } else {
                                    Node(e, ~l, ~insert(r, elem))
                                }
        }
    }
    list.move_iter().fold(Empty, |t, e| insert(t, e))
}

fn main() {
    assert!(construct(~[3, 2, 5, 7, 1]) ==
            Node(3, ~Node(2, ~Node(1, ~Empty, ~Empty), ~Empty),
                    ~Node(5, ~Empty, ~Node(7, ~Empty, ~Empty))));
}
