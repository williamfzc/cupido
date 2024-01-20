use tracing::info;
use cupid::collector::{Config, walk};
use cupid::server::{server_main, ServerConfig};

fn main() {
    tracing_subscriber::fmt::init();

    info!("graph creating ...");
    let conf = Config::new(".", 10240);
    let graph = walk(conf);
    info!("graph ready: {:?}", graph.size());

    let server_conf = ServerConfig::new(graph);
    info!("server up");
    server_main(server_conf);
}
