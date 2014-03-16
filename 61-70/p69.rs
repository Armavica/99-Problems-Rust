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

// BNF syntax:
// label ::= 'a'-'z'
// node  ::= <label> <node> <node> | '.'

fn tree_to_dotstring(tree: BinaryTree<char>) -> ~str {
    match tree {
        Node(c, ~left, ~right) => std::str::from_char(c) + tree_to_dotstring(left) + tree_to_dotstring(right),
        Empty => ~"."
    }
}

fn dotstring_to_tree(dotstring: ~str) -> BinaryTree<char> {
    fn aux(dotstring: &mut std::str::Chars) -> BinaryTree<char> {
        match dotstring.next() {
            Some('.') => Empty,
            Some( c ) => Node(c, ~aux(dotstring), ~aux(dotstring)),
            None      => fail!("invalid argument for dotstring_to_tree")
        }
    }
    aux(&mut dotstring.chars())
}

fn main() {
    let tree = Node('a', ~Node('b', ~Node('d', ~Empty, ~Empty),
                                    ~Node('e', ~Empty, ~Empty)),
                         ~Node('c', ~Empty,
                                    ~Node('f', ~Node('g', ~Empty, ~Empty), ~Empty)));
    assert_eq!(tree_to_dotstring(tree.clone()), ~"abd..e..c.fg...");
    assert_eq!(dotstring_to_tree(~"abd..e..c.fg..."), tree);
}

