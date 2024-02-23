enum NodeType {
    Input,
    Function,
    Product
}

struct Node {
    name: String,
    node_type: NodeType,
    input: Option<String>,
    output: Option<String>,
}

struct DAG {
    nodes: Vec<Node>,
    relations: Vec<(String, String)>,
}

impl DAG {
    fn new() -> DAG {
        DAG {
            nodes: Vec::new(),
            relations: Vec::new(),
        }
    }

    fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    fn add_relation(&mut self, from: String, to: String) {
        self.relations.push((from, to));
    }
}