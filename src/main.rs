use cupid::collector::{walk, Config};
use cupid::server::{server_main, ServerConfig};
use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();

    info!("graph creating ...");
    let conf = Config::default();
    let graph = walk(conf);
    info!("graph ready: {:?}", graph.size());

    let server_conf = ServerConfig::new(graph);
    info!("server up");
    server_main(server_conf);
}
