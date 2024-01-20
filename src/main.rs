use clap::Command;
use cupid::collector::{walk, Config};
use cupid::server::{server_main, ServerConfig};
use tracing::info;

fn main() {
    let cmd = Command::new("cupid")
        .bin_name("cupid")
        .subcommand_required(true)
        .subcommand(Command::new("up").about("cupid server up"));
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("up", _matches)) => handle_up(),
        _ => unreachable!("clap should ensure we don't get here"),
    };
}

fn handle_up() {
    tracing_subscriber::fmt::init();

    info!("graph creating ...");
    let conf = Config::default();
    let graph = walk(conf);
    info!("graph ready: {:?}", graph.size());

    let server_conf = ServerConfig::new(graph);
    info!("server up");
    server_main(server_conf);
}
