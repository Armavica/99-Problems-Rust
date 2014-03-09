#[deriving(Clone)]
enum BinaryTree<T> {
    Node(T, ~BinaryTree<T>, ~BinaryTree<T>),
    Empty
}

fn cbal_tree_func(n: uint) -> ~[BinaryTree<char>] {
    match n {
        0 => ~[Empty],
        _ if n % 2 == 1 => {
            let t = cbal_tree_func(n/2);
            t.flat_map(|s| t.iter().map(|u| Node('x', ~s.clone(), ~u.clone())).to_owned_vec())
        }
        _ => {
            let a = cbal_tree_func(n/2 - 1);
            let b = cbal_tree_func(n/2);
            let lr = a.flat_map(|s| b.iter().map(|u| Node('x', ~s.clone(), ~u.clone())).to_owned_vec());
            let rl = b.flat_map(|s| a.iter().map(|u| Node('x', ~s.clone(), ~u.clone())).to_owned_vec());
            std::vec::append(lr, rl)
        }
    }
}

fn cbal_tree_iter(n: uint) -> ~[BinaryTree<char>] {
    match n {
        0 => ~[Empty],
        _ if n % 2 == 1 => {
            let t = cbal_tree_iter(n/2);
            let mut r = ~[];
            for s in t.iter() {
                for u in t.iter() {
                    r.push(Node('x', ~s.clone(), ~u.clone()));
                }
            }
            r
        }
        _ => {
            let a = cbal_tree_iter(n/2 - 1);
            let b = cbal_tree_iter(n/2);
            let mut r = ~[];
            for s in a.move_iter() {
                for u in b.iter() {
                    r.push(Node('x', ~s.clone(), ~u.clone()));
                    r.push(Node('x', ~u.clone(), ~s.clone()));
                }
            }
            r
        }
    }
}


fn main() {
    assert!(cbal_tree_func(40).len() == 524288);
    assert!(cbal_tree_iter(40).len() == 524288);
}
