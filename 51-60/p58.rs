#[deriving(Clone)]
enum BinaryTree<T> {
    Node(T, ~BinaryTree<T>, ~BinaryTree<T>),
    Empty
}

fn is_mirror<T>(t1: &BinaryTree<T>, t2: &BinaryTree<T>) -> bool {
    match (t1, t2) {
        (&Empty, &Empty) => true,
        (&Node(_, ~ref t1l, ~ref t1r), &Node(_, ~ref t2l, ~ref t2r)) => is_mirror(t1l, t2r) && is_mirror(t1r, t2l),
        _ => false
    }
}

fn is_symmetric<T>(tree: &BinaryTree<T>) -> bool {
    match *tree {
        Empty => true,
        Node(_, ~ref l, ~ref r) => is_mirror(l, r)
    }
}

fn cbal_tree(n: uint) -> ~[BinaryTree<char>] {
    match n {
        0 => ~[Empty],
        _ if n % 2 == 1 => {
            let t = cbal_tree(n/2);
            t.flat_map(|s| t.iter().map(|u| Node('x', ~s.clone(), ~u.clone())).to_owned_vec())
        }
        _ => {
            let a = cbal_tree(n/2 - 1);
            let b = cbal_tree(n/2);
            let lr = a.flat_map(|s| b.iter().map(|u| Node('x', ~s.clone(), ~u.clone())).to_owned_vec());
            let rl = b.flat_map(|s| a.iter().map(|u| Node('x', ~s.clone(), ~u.clone())).to_owned_vec());
            std::vec::append(lr, rl)
        }
    }
}

fn sym_cbal_trees(n: uint) -> ~[BinaryTree<char>] {
    cbal_tree(n).move_iter().filter(|t| is_symmetric(t)).to_owned_vec()
}



fn main() {
    assert!(sym_cbal_trees(57).len() == 256);
}
