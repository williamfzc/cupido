use crate::graph::{CupidGraph, GraphSize};
use axum::routing::get;
use axum::Router;
use std::sync::{Arc, RwLock};
use axum::extract::Query;
use serde_derive::{Deserialize, Serialize};
use tracing::error;

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

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
pub async fn server_main(server_conf: ServerConfig) {
    *SERVER_CONFIG.write().unwrap() = server_conf;

    let app = Router::new()
        .route("/size", get(size_handler))
        .route("/", get(root_handler))
        .route("/file/related/commit", get(file_related_commit));

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

async fn root_handler() -> axum::Json<Desc> {
    axum::Json(Desc {
        version: VERSION.to_string(),
    })
}

async fn file_related_commit(Query(params): Query<Params>) -> axum::Json<Vec<String>> {
    let conf = SERVER_CONFIG.read().unwrap();
    return match conf.graph.related_commits(&params.file) {
        Ok(commits) => {
            axum::Json(commits)
        }
        Err(error) => {
            error!("file_related_commit error: {}", error);
            axum::Json(Vec::new())
        }
    };
}

#[derive(Debug, Deserialize)]
struct Params {
    file: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Desc {
    version: String,
}
