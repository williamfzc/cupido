use cupid::server::{server_main, ServerConfig};
use cupid::{walk, Config};

fn main() {
    let conf = Config::new(".", 10240);
    let graph = walk(conf);

    let server_conf = ServerConfig::new(graph);
    server_main(server_conf);
}
