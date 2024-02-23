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