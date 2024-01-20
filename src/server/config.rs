use crate::graph::CupidGraph;

pub struct ServerConfig {
    pub port: u16,
    pub graph: CupidGraph,
}

impl ServerConfig {
    pub fn new(cupid_graph: CupidGraph) -> ServerConfig {
        return ServerConfig {
            port: 9410,
            graph: cupid_graph,
        };
    }
}
