use graphlive::{InternetOfThings};

fn main() {
    let mut graph = InternetOfThings::new();
    println!("{:?}", graph);
    graph.set_nodes(vec!['A','B','C']);
    println!("{:?}", graph);
    graph.set_edges('A', vec![(10,'A'),(5, 'B'), (2, 'C')]);
    graph.set_edges('B', vec![(4, 'A'), (1, 'C')]);
    graph.set_edges('C', vec![(5, 'A'), (10, 'B')]);
    graph.set_edges('D', vec![(1, 'C')]);
    println!("{:?}", graph);

    graph.direct_neighbors('A');
    graph.direct_neighbors('C');
}
