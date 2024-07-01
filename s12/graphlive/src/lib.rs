type KeyType = char;

#[derive(Debug, Clone)]
struct Edge {
    weight: u32,
    node: usize,
}


#[derive(Debug)]
pub struct InternetOfThings {
    adjacency_list: Vec<Vec<Edge>>,
    nodes: Vec<KeyType>,
}


impl InternetOfThings {
    pub fn new() -> InternetOfThings {
        InternetOfThings { adjacency_list: Vec::new() , nodes: Vec::new() }
    }

    fn get_node_index(&self, node: KeyType) -> Option<usize> {
        self.nodes.iter().position(|n| n == &node)
    }

    pub fn edges(&self) -> u64 {
        self.adjacency_list.iter().fold(0u64, |p, c| p + c.len() as u64)
    }

    pub fn nodes(&self) -> usize {
        self.nodes.len()
    }


    pub fn set_nodes(&mut self, nodes: Vec<KeyType>) {
        self.nodes = nodes;
        self.adjacency_list = vec![vec![]; self.nodes.len()]
    }

    pub fn set_edges(&mut self, from: KeyType, edges: Vec<(u32, KeyType)>) {
        // filter edges
        let edges: Vec<Edge> = edges.into_iter().filter_map(|e| {
            if let Some(to) = self.get_node_index(e.1) {
                Some(Edge { weight: e.0, node: to })
                } else {
                    None
                }}).collect();

        // populate and/or create adjacency list for node
        match self.nodes.iter().position(|n| n == &from) {
            Some(i) => self.adjacency_list[i] = edges,
            None => {
                self.nodes.push(from);
                self.adjacency_list.push(edges)
            }
        }
    }

    pub fn direct_neighbors(&self, node: KeyType) {
        // get index of node in self.nodes
        let index = self.nodes.iter().position(|n| n == &node).expect("NOT FOUND");
        // get list of edges in self.adjacency_list
        let edges = &self.adjacency_list[index];
        // print self.nodes for each Edge.node
        // [A(10), B(2), C(3)]
        let formatted_edges: Vec<String> = edges.iter().map(|edge| {
            format!("{}({})", self.nodes[edge.node], edge.weight)
        }).collect();

        println!("{:?}", formatted_edges)
    }
}
