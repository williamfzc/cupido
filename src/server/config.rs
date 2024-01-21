use crate::graph::RelationGraph;

pub struct ServerConfig {
    pub port: u16,
    pub graph: RelationGraph,
}

impl ServerConfig {
    pub fn new(cupido_graph: RelationGraph) -> ServerConfig {
        return ServerConfig {
            port: 9410,
            graph: cupido_graph,
        };
    }
}
