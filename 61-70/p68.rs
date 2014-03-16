#[deriving(Clone)]
#[deriving(Eq)]
enum BinaryTree<T> {
    Node(T, ~BinaryTree<T>, ~BinaryTree<T>),
    Empty
}

impl std::fmt::Show for BinaryTree<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn aux(tree: BinaryTree<char>) -> ~str {
            match tree {
                Empty => ~"",
                Node(c, ~Empty, ~Empty) => std::str::from_char(c),
                Node(c, ~l, ~r) => std::str::from_char(c)
                                    + "(" + aux(l) + "," + aux(r) + ")"
            }
        }
        aux(self.clone()).fmt(f)
    }
}

fn preorder(tree: BinaryTree<char>) -> ~str {
    match tree {
        Node(c, ~left, ~right) => std::str::from_char(c) + preorder(left) + preorder(right),
        Empty => ~""
    }
}

fn inorder(tree: BinaryTree<char>) -> ~str {
    match tree {
        Node(c, ~left, ~right) => inorder(left) + std::str::from_char(c) + inorder(right),
        Empty => ~""
    }
}

fn pre_in_tree(preorder: ~str, inorder: ~str) -> BinaryTree<char> {
    fn aux(head:        Option<char>,
           preord:      &mut std::str::Chars,
           first_inord: Option<char>,
           inord:       &mut std::str::Chars
          )             -> BinaryTree<char> {
        let (left, header) =
        match (preord.next(), first_inord.or_else(|| inord.next())) {
            (Some(p), Some(i)) if p == i => (Empty, p),
            (Some(p), Some(i)) => (aux(Some(p), preord, Some(i), inord), p),
            (None, _) => return Empty,
            _ => fail!("invalid arguments for pre_in_tree")
        };
        match (head, inord.next()) {
            (Some(h), Some(n)) if h == n => Node(header, ~left, ~Empty),
            (_, Some(n)) => Node(header, ~left, ~aux(head, preord, Some(n), inord)),
            (_, None) => Node(header, ~left, ~Empty),
        }
    }
    aux(None, &mut preorder.chars(), None, &mut inorder.chars())
}

fn main() {
    let tree = Node('a', ~Node('b', ~Node('d', ~Empty, ~Empty),
                                    ~Node('e', ~Empty, ~Empty)),
                         ~Node('c', ~Empty,
                                    ~Node('f', ~Node('g', ~Empty, ~Empty), ~Empty)));
    assert_eq!(preorder(tree.clone()), ~"abdecfg");
    assert_eq!(inorder(tree.clone()), ~"dbeacgf");
    assert_eq!(pre_in_tree(preorder(tree.clone()), inorder(tree.clone())), tree);

    let tree = Empty;
    assert_eq!(pre_in_tree(preorder(tree.clone()), inorder(tree.clone())), tree);

    let tree = Node('a', ~Node('b', ~Node('c', ~Empty, ~Empty), ~Empty), ~Empty);
    assert_eq!(pre_in_tree(preorder(tree.clone()), inorder(tree.clone())), tree);

    let tree = Node('a', ~Empty, ~Node('b', ~Empty, ~Node('c', ~Empty, ~Empty)));
    assert_eq!(pre_in_tree(preorder(tree.clone()), inorder(tree.clone())), tree);

    let tree = Node('a', ~Node('b', ~Empty, ~Empty), ~Node('c', ~Empty, ~Empty));
    assert_eq!(pre_in_tree(preorder(tree.clone()), inorder(tree.clone())), tree);

    let tree = Node('a', ~Node('b', ~Node('c', ~Empty, ~Empty), ~Empty), ~Node('d', ~Empty, ~Empty));
    assert_eq!(pre_in_tree(preorder(tree.clone()), inorder(tree.clone())), tree);
}

