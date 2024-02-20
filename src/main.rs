mod args;
use args::GraphyArgs;
use clap::Parser;

struct Edge {
    node: Node,
    label: String,
}

struct Node {
    // id is a unique identifier into DB of items
    id: String,
    edges: Vec<Edge>,
}

impl Node {
    fn new(id: String) -> Self {
        Self {
            id,
            edges: Vec::new(),
        }
    }
}

fn main() {
    // let args = GraphyArgs::parse();
    // let path = args.db_path;
}
