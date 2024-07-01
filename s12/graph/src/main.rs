use graph::{InternetOfThings};

fn main() {
    let mut graph = InternetOfThings::new();
    println!("{:?}", graph);
    graph.set_nodes(vec![0,1,2,3]);
    println!("{:?}", graph);
    graph.set_edges(1, vec![(10,1),(5, 2), (2, 3)]);
    graph.set_edges(2, vec![(4, 1), (1, 3)]);
    println!("{:?}", graph);
}
