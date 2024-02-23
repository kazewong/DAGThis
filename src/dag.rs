enum NodeType {
    Input,
    Function,
    Product
}

struct Node {
    name: String,
    node_type: NodeType,
    data: Option<String>,
}

struct DAG {
    nodes: Vec<Node>,
    relations: Vec<(String, String)>,
}