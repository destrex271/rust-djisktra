
use djisktra::Graph;

fn main(){
    let mut graph = Graph::new();
    graph.add_node(10);
    graph.add_edge(10, 12, 2);
    graph.add_node(2);
    graph.add_edge(10, 6, 5);
    graph.add_node(5);
    graph.add_node(23);
    graph.add_edge(2, 7, 23);
    graph.add_edge(5, 3, 23);

    graph.add_node(34);
    graph.add_edge(2, 5, 34);

    graph.traverse_graph();

    println!("{:?}", graph);
}
