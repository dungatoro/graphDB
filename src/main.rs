mod args;
use args::GraphyArgs;
use clap::Parser;

struct Node {
    // id is a unique identifier into DB of items
    id: String,
    edges: Vec<Node>
}

impl Node {
    fn new(id: String) -> Self {
        Self { id, edges: Vec::new() }
    }
}

fn main() {
    
}
