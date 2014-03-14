#[deriving(Clone)]
#[deriving(Eq)]
enum BinaryTree<T> {
    Node(T, ~BinaryTree<T>, ~BinaryTree<T>),
    Empty
}

//struct BinaryTreeStr {string: ~str}

//trait BinTree {
    //fn tree_to_str(tree: BinaryTree<char>) -> BinaryTreeStr;
    //fn str_to_tree(string: ~str) -> BinaryTree<char>;
//}

fn tree_to_str(tree: BinaryTree<char>) -> ~str {
    match tree {
        Empty => ~"",
        Node(c, ~Empty, ~Empty) => std::str::from_char(c),
        Node(c, ~l, ~r) => std::str::from_char(c)
                            + "(" + tree_to_str(l) + "," + tree_to_str(r) + ")"
    }
}

fn str_to_tree(string: ~str) -> BinaryTree<char> {
    fn aux(string: &mut std::str::Chars) -> BinaryTree<char> {
        let mut head = None;
        let mut left = Empty;
        let mut right = Empty;
        loop {
            match string.next() {
                Some(',') | Some(')') | None =>
                    return match head {
                        None => Empty,
                        Some(c) => Node(c, ~left, ~right)
                    },
                Some('(') => {left = aux(string); right = aux(string)}
                Some(c)   => head = Some(c),
            }
        }
    }
    aux(&mut string.chars())
}


fn main() {
    let tree = Node('a', ~Node('b', ~Node('d', ~Empty, ~Empty),
                                    ~Node('e', ~Empty, ~Empty)),
                         ~Node('c', ~Empty,
                                    ~Node('f', ~Node('g', ~Empty, ~Empty), ~Empty)));

    assert_eq!(tree_to_str(tree.clone()), ~"a(b(d,e),c(,f(g,)))");
    assert!(str_to_tree(~"a(b(d,e),c(,f(g,)))") == tree);
}

