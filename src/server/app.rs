use crate::graph::CupidGraph;
use crate::server::config::ServerConfig;
use crate::server::handler::{create_router};
use std::sync::{Arc, RwLock};

lazy_static::lazy_static! {
    pub static ref SERVER_CONFIG: Arc<RwLock<ServerConfig>> = Arc::new(RwLock::new(ServerConfig::new(CupidGraph::new())));
}

pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
pub async fn server_main(server_conf: ServerConfig) {
    *SERVER_CONFIG.write().unwrap() = server_conf;

    let routers = create_router();

    let listener =
        tokio::net::TcpListener::bind(format!("127.0.0.1:{}", SERVER_CONFIG.read().unwrap().port))
            .await
            .unwrap();
    axum::serve(listener, routers).await.unwrap();
}
