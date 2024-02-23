# DAGThis

This library do one thing and one thing only: given a directory, scan all the files underneath recursively to search for predefined marco entry points, then building a Directed acyclic graph (DAG) to describe the dependencies structure of the entry points.

## Roadmap

- [ ] Basic graph structure
- [ ] Building Dag given entry points
- [ ] Plain text scanning
- [ ] Python file scanning
  - Check only for functions or classes with decorators. See [this](https://stackoverflow.com/questions/68583870/checking-whether-a-function-is-decorated)
  - Only support function and classes. The work should be done in the decorator. We don't want to run the full file to get the entry points.
- [ ] Latex file scanning
- [ ] Programmatically add nodes and edges to the graph
- [ ] Render the graph to a file