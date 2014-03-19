use std::vec_ng::Vec;

#[deriving(Eq)]
#[deriving(Show)]
enum MultiTree<'a, T> {
    Tree(T, std::vec_ng::Vec<MultiTree<'a, T>>)
}

fn count_nodes<T>(tree: MultiTree<T>) -> uint {
    match tree {
        Tree(_, f) => f.move_iter().fold(1, |sum, t| sum + count_nodes(t))
    }
}

fn tree_to_string(tree: MultiTree<char>) -> ~str {
    match tree {
        Tree(c, forest) => {
            std::str::from_char(c) + 
                forest.move_iter().map(|t| tree_to_string(t))
                                  .to_owned_vec().concat() + "^"
        }
    }
}

// EBNF Grammar:
// node ::= 'a'-'z'
// tree ::= node tree* '^'

fn string_to_tree(string: ~str) -> MultiTree<char> {
    fn aux(string: &mut std::str::Chars) -> Option<MultiTree<char>> {
        match string.next() {
            Some('^') => None,
            Some(c) => {
                let mut forest = Vec::new();
                loop { match aux(string) {
                    Some(subtree) => forest.push(subtree),
                    None          => return Some(Tree(c, forest))
                } }
            }
            None    => unreachable!()
        }
    }
    aux(&mut string.chars()).unwrap()
}

#[test]
fn p70_count_nodes_one() {
    let tree = Tree(42, std::vec_ng::Vec::new());
    assert_eq!(count_nodes(tree), 1);
}

#[test]
fn p70_count_nodes_tree() {
    let tree = Tree('a', vec!(Tree('f', vec!(Tree('g', vec!()))),
                              Tree('c', vec!()),
                              Tree('b', vec!(Tree('d', vec!()),
                                             Tree('e', vec!())))));
    assert_eq!(count_nodes(tree), 7);
}

#[test]
fn p70_tree_to_string() {
    let tree = Tree('a', vec!(Tree('f', vec!(Tree('g', vec!()))),
                              Tree('c', vec!()),
                              Tree('b', vec!(Tree('d', vec!()),
                                             Tree('e', vec!())))));
    assert_eq!(tree_to_string(tree), ~"afg^^c^bd^e^^^");
}

#[test]
fn p70_string_to_tree() {
    let tree = Tree('a', vec!(Tree('f', vec!(Tree('g', vec!()))),
                              Tree('c', vec!()),
                              Tree('b', vec!(Tree('d', vec!()),
                                             Tree('e', vec!())))));
    assert_eq!(string_to_tree(~"afg^^c^bd^e^^^"), tree);
}
