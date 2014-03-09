enum BinaryTree<T> {
    Node(T, ~BinaryTree<T>, ~BinaryTree<T>),
    Empty
}

fn min_nodes(h: uint) -> uint {
    fn aux(h: uint, (acc1, acc0): (uint, uint)) -> uint {
        match h {
            0 => acc0,
            1 => acc1,
            _ => aux(h-1, (acc1 + acc0 + 1, acc1))
        }
    }
    aux(h, (1, 0))
}

fn max_height(n: uint) -> uint {
    std::iter::count(0u, 1).skip_while(|&i| min_nodes(i) <= n).next().unwrap() - 1
}

fn hbal_tree_nodes<T>(n: uint) -> ~[BinaryTree<T>] {
    unimplemented!()
}


fn main() {
    assert_eq!(min_nodes(42), 701408732);
    assert_eq!(max_height(701408732), 42);
    assert_eq!(max_height(701408731), 41);
    assert_eq!(max_height(1<<63), 90);
}

