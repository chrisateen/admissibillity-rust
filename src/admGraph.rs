use graphbench::editgraph::EditGraph;
use graphbench::graph::VertexSet;

pub struct AdmGraph {
    graph: EditGraph,
    r: VertexSet,
    l: VertexSet,
    candidates: VertexSet
}