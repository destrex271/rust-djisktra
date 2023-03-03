use std::collections::HashMap;

#[derive(Debug)]
pub struct Node{
    pub weight: usize,
    pub edges: Vec<Edge>
}

#[derive(Debug)]
pub struct Edge{
    pub cost: usize,
    pub to: usize
}

#[derive(Debug)]
pub struct Graph{
    pub nodes: HashMap<usize, Node>
}

impl Graph{
    pub fn new() -> Graph{
        Graph { nodes: HashMap::new() }
    }
    pub fn add_node(&mut self, weight: usize){
        self.nodes.insert(weight, Node{weight,edges: vec![]});
    }
    // Directed Graph
    pub fn add_edge(&mut self, from: usize, cost: usize, to: usize){
        let x = self.nodes.get_mut(&from);
        let edge = Edge{cost, to};
        x.unwrap().edges.push(edge);
    }
    pub fn traverse_graph(&mut self){
        for (id, node) in self.nodes.iter(){
            println!("{:?} : {:?}", id, node);
        }
    }
}
