use clap::{Parser};
use cupido::collector::{walk, Config};
use cupido::server::app::server_main;
use cupido::server::config::ServerConfig;
use tracing::info;

#[derive(Parser, Debug)]
#[clap(name = "cupido", bin_name = "cupido", about = "Cupido command line tool")]
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
}

fn main() {
    let cli: Cli = Cli::parse();

    match cli.cmd {
        SubCommand::Up(up_cmd) => handle_up(up_cmd),
    }
}

fn handle_up(up_cmd: UpCommand) {
    tracing_subscriber::fmt::init();

    info!("graph creating ...");
    let mut conf = Config::default();
    if let Some(ref user_issue_regex) = up_cmd.issue_regex {
        conf.issue_regex = user_issue_regex.clone()
    }
    if let Some(ref repo_path) = up_cmd.repo_path {
        conf.repo_path = repo_path.clone()
    }

    info!("config: {:?}", up_cmd);
    let graph = walk(conf);
    info!("graph ready: {:?}", graph.size());

    let server_conf = ServerConfig::new(graph);
    info!("server up");
    server_main(server_conf);
}
