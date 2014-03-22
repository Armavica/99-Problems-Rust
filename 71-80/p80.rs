#[allow(deprecated_owned_vector)];
extern crate collections;
use collections::HashSet;
use collections::HashMap;
use std::hash::Hash;
use std::fmt::Show;

#[deriving(Clone)]
struct GraphTerm<T> {
    nodes: HashSet<T>,
    edges: HashSet<(T, T)>
}

#[deriving(Clone)]
struct AdjacencyList<T> {
    nodes: HashMap<T, HashSet<T>>
}

trait Graph<T> : Show {
    fn to_graph_term(&self) -> GraphTerm<T>;
    fn to_adjacency_list(&self) -> AdjacencyList<T>;
}

impl<T: Eq+Hash> Eq for GraphTerm<T> {
    fn eq(&self, other: &GraphTerm<T>) -> bool {
        self.nodes == other.nodes && self.edges == other.edges
    }
}

impl<T: Eq+Hash+Show+Clone> Graph<T> for GraphTerm<T> {
    fn to_graph_term(&self) -> GraphTerm<T> {
        self.clone()
    }

    fn to_adjacency_list(&self) -> AdjacencyList<T> {
        let mut nodes: HashMap<T, HashSet<T>> = HashMap::new();
        let mut in_edges = HashSet::new(); // nodes involved in at least one edge
        for &(ref a, ref b) in self.edges.iter() {
            in_edges.insert(a.clone());
            in_edges.insert(b.clone());
            nodes.insert_or_update_with(a.clone(),
                                        vec!(b.clone()).move_iter().collect(),
                                        |_, set| {set.insert(b.clone());} );
            nodes.find_or_insert(b.clone(),
                                 HashSet::new());
        }
        for a in self.nodes.difference(&in_edges) {
            nodes.insert(a.clone(), HashSet::new());
        }
        AdjacencyList { nodes: nodes }
    }
}

impl<T: Eq+Hash+Show+Clone> Graph<T> for AdjacencyList<T> {
    fn to_graph_term(&self) -> GraphTerm<T> {
        let mut nodes = HashSet::new();
        let mut edges = HashSet::new();
        for (node, adj) in self.nodes.iter() {
            nodes.insert(node.clone());
            edges.extend(&mut adj.iter().map(|o| (node.clone(), o.clone())));
        }
        GraphTerm { nodes: nodes, edges: edges }
    }

    fn to_adjacency_list(&self) -> AdjacencyList<T> {
        self.clone()
    }
}

impl<T: Eq+Hash+Show+Clone> Show for GraphTerm<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut in_edges = HashSet::new(); // nodes involved in at least one edge
        let edges: ~[~str] = self.edges.iter().map(|&(ref a, ref b)| {
                                          in_edges.insert(a.clone());
                                          in_edges.insert(b.clone());
                                          format!("{}-{}", a, b)})
                                     .collect();
        let nodes: ~[~str] = self.nodes.difference(&in_edges)
                              .map(|ref a| format!("{}", a))
                              .collect();
        std::vec::append(edges, nodes).connect(" ").fmt(f)
    }
}

impl<T: Eq+Hash+Show> Show for AdjacencyList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let edges: ~[~str] = self.nodes.iter().map(|(ref a, ref adj)|
                                                   if adj.is_empty() {
                                                       format!("{}", a)
                                                   } else {
                                                        adj.iter().map(|ref b|
                                                            format!("{}-{}", a, b))
                                                            .to_owned_vec()
                                                            .connect(" ")
                                                   }
                                                    ).collect();
        edges.connect(" ").fmt(f)
    }
}

#[test]
fn p80_gt_to_al_to_gt() {
    let n: HashSet<char> = vec!('b', 'c', 'd', 'f', 'g', 'h', 'k')
                           .move_iter().collect();
    let e: HashSet<(char, char)> = vec!(('h', 'g'),
                                        ('k', 'f'),
                                        ('f', 'b'),
                                        ('f', 'c'),
                                        ('c', 'b'))
                                   .move_iter().collect();
    let gt = GraphTerm {nodes: n, edges: e};
    let gt2 = gt.clone().to_adjacency_list().to_graph_term();
    assert_eq!(gt, gt2);
}

