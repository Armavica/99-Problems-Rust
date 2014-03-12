enum BinaryTree<T> {
    Node(T, ~BinaryTree<T>, ~BinaryTree<T>),
    Empty
}

enum PosBinaryTree<T> {
    PosNode(T, (int, int), ~PosBinaryTree<T>, ~PosBinaryTree<T>),
    PosEmpty
}

impl<T: Eq> Eq for PosBinaryTree<T> {
    fn eq(&self, other: &PosBinaryTree<T>) -> bool {
        match (self, other) {
            (&PosEmpty, &PosEmpty) => true,
            (&PosNode(ref lv, ref posl, ~ref ll, ~ref lr),
             &PosNode(ref rv, ref posr, ~ref rl, ~ref rr)) => lv == rv &&
                                                              posl == posr &&
                                                              ll == rl &&
                                                              lr == rr,
            _ => false
        }
    }
}

fn layout_binary_tree<T: Clone>(tree: &BinaryTree<T>) -> PosBinaryTree<T> {
    fn aux<T: Clone>(leftmost: int, level: uint, subtree: &BinaryTree<T>) -> (int, PosBinaryTree<T>) {
        match subtree {
            &Empty => (leftmost, PosEmpty),
            &Node(ref e, ~ref l, ~ref r) => {
                let (middle, leftTree) = aux(leftmost, level + 1, l);
                let (rightmost, rightTree) = aux(middle + 1, level + 1, r);
                (rightmost, PosNode(e.clone(), (level as int, middle + 1), ~leftTree, ~rightTree))
            }
        }
    }
    let (_, r) = aux(0, 1, tree);
    r
}

fn main() {
    let tree = Node('n', ~Node('k', ~Node('c', ~Node('a', ~Empty,
                                                          ~Empty),
                                               ~Node('h', ~Node('g', ~Node('e', ~Empty,
                                                                                ~Empty),
                                                                     ~Empty),
                                                          ~Empty)),
                                    ~Node('m', ~Empty, ~Empty)),
                         ~Node('u', ~Node('p', ~Empty,
                                               ~Node('s', ~Node('q', ~Empty,
                                                                     ~Empty),
                                                          ~Empty)),
                                    ~Empty));
    let postree = PosNode('n', (1, 8), ~PosNode('k', (2, 6), ~PosNode('c',
        (3, 2), ~PosNode('a', (4, 1), ~PosEmpty, ~PosEmpty), ~PosNode('h',
        (4, 5), ~PosNode('g', (5, 4), ~PosNode('e', (6, 3), ~PosEmpty, ~PosEmpty),
        ~PosEmpty), ~PosEmpty)), ~PosNode('m', (3, 7), ~PosEmpty, ~PosEmpty)),
        ~PosNode('u', (2, 12), ~PosNode('p', (3, 9), ~PosEmpty, ~PosNode('s',
        (4, 11), ~PosNode('q', (5, 10), ~PosEmpty, ~PosEmpty), ~PosEmpty)), ~PosEmpty));

    assert!(layout_binary_tree(&tree) == postree);
}

