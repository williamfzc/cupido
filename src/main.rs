use clap::Parser;
use cupido::collector::config::get_collector;
use cupido::collector::config::Collect;
use cupido::collector::config::Config;
use cupido::server::app::server_main;
use cupido::server::config::ServerConfig;
use std::time::Instant;
use tracing::info;

#[derive(Parser, Debug)]
#[clap(
    name = "cupido",
    bin_name = "cupido",
    about = "Cupido command line tool"
)]
struct Cli {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Cupido server up
    #[clap(name = "up")]
    Up(UpCommand),
}

#[derive(Parser, Debug)]
struct UpCommand {
    /// For catching issues in commit message
    #[clap(short, long)]
    issue_regex: Option<String>,

    /// Root location
    #[clap(short, long)]
    repo_path: Option<String>,

    /// File include
    #[clap(short, long)]
    path_specs: Option<String>,
}

fn main() {
    let cli: Cli = Cli::parse();

    match cli.cmd {
        SubCommand::Up(up_cmd) => handle_up(up_cmd),
    }
}

fn handle_up(up_cmd: UpCommand) {
    tracing_subscriber::fmt::init();

    info!("relation creating ...");
    let mut conf = Config::default();
    if let Some(ref user_issue_regex) = up_cmd.issue_regex {
        conf.issue_regex = user_issue_regex.clone()
    }
    if let Some(ref repo_path) = up_cmd.repo_path {
        conf.repo_path = repo_path.clone()
    }
    if let Some(ref path_specs) = up_cmd.path_specs {
        conf.path_specs = path_specs.split(";").map(|a| a.into()).collect();
    }

    info!("config: {:?}", up_cmd);
    let start_time = Instant::now();

    let collector = get_collector();
    let graph = collector.walk(conf);
    info!(
        "relation ready in {:?}: {:?}",
        start_time.elapsed(),
        graph.size()
    );

    let server_conf = ServerConfig::new(graph);
    info!("server up");
    server_main(server_conf);
}
