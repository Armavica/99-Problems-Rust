#[deriving(Clone)]
enum BinaryTree<T> {
    Node(T, ~BinaryTree<T>, ~BinaryTree<T>),
    Empty
}

fn hbal_tree(n: uint) -> ~[BinaryTree<char>] {
    match n {
        0 => ~[Empty],
        1 => ~[Node('x', ~Empty, ~Empty)],
        _ => {
            let p2 = hbal_tree(n-2);
            let p1 = hbal_tree(n-1);
            let mut r = ~[];
            for t in p1.iter() {
                for s in p2.iter() {
                    r.push(Node('x', ~t.clone(), ~s.clone()));
                    r.push(Node('x', ~s.clone(), ~t.clone()));
                }
                for s in p1.iter() {
                    r.push(Node('x', ~t.clone(), ~s.clone()));
                }
            }
            r
        }
    }
}


fn main() {
    assert!(hbal_tree(3).len() == 15);
}
