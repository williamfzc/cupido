use axum::routing::get;
use axum::Router;
use std::sync::{Arc, RwLock};
use crate::graph::{CupidGraph, GraphSize};

pub struct ServerConfig {
    port: u16,
    graph: CupidGraph,
}

impl ServerConfig {
    pub fn new(cupid_graph: CupidGraph) -> ServerConfig {
        return ServerConfig {
            port: 9410,
            graph: cupid_graph,
        };
    }
}

lazy_static::lazy_static! {
    static ref SERVER_CONFIG: Arc<RwLock<ServerConfig>> = Arc::new(RwLock::new(ServerConfig::new(CupidGraph::new())));
}

#[tokio::main]
pub async fn server_main(server_conf: ServerConfig) {
    *SERVER_CONFIG.write().unwrap() = server_conf;

    let app = Router::new().route("/size", get(size_handler));

    let listener =
        tokio::net::TcpListener::bind(format!("127.0.0.1:{}", SERVER_CONFIG.read().unwrap().port))
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn size_handler() -> axum::Json<GraphSize> {
    let conf = SERVER_CONFIG.read().unwrap();
    axum::Json(conf.graph.size())
}
