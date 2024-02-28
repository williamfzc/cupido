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

    /// Extract file based mapping
    #[clap(name = "map")]
    Map(MapCommand),
    // Diff (subset of `Extract` commands
    // TODO
}

#[derive(Parser, Debug)]
struct CommonOptions {
    /// For catching issues in commit message
    #[clap(short, long)]
    issue_regex: Option<String>,

    /// Root location
    #[clap(short, long)]
    repo_path: Option<String>,

    /// File include
    #[clap(short, long)]
    path_specs: Option<String>,

    /// Multi parents search
    #[clap(short, long)]
    multi_parents: Option<bool>,

    /// Show progress
    #[clap(long)]
    progress: Option<bool>,
}

#[derive(Parser, Debug)]
struct UpCommand {
    #[clap(flatten)]
    common_options: CommonOptions,

    #[clap(long)]
    port: Option<u16>,
}

#[derive(Parser, Debug)]
struct MapCommand {
    #[clap(flatten)]
    common_options: CommonOptions,
}

fn main() {
    let cli: Cli = Cli::parse();

    match cli.cmd {
        SubCommand::Up(up_cmd) => handle_up(up_cmd),
        SubCommand::Map(map_cmd) => handle_map(map_cmd),
    }
}

fn handle_map(map_command: MapCommand) {
    info!("relation creating ...");
    let mut conf = Config::default();
    if let Some(ref user_issue_regex) = map_command.common_options.issue_regex {
        conf.issue_regex = user_issue_regex.clone()
    }
    if let Some(ref repo_path) = map_command.common_options.repo_path {
        conf.repo_path = repo_path.clone()
    }
    if let Some(ref path_specs) = map_command.common_options.path_specs {
        conf.path_specs = path_specs.split(";").map(|a| a.into()).collect();
    }
    if let Some(ref multi_parents) = map_command.common_options.multi_parents {
        conf.multi_parents = multi_parents.clone()
    }

    info!("config: {:?}", map_command);
    let start_time = Instant::now();

    let collector = get_collector();
    let graph = collector.walk(conf);
    info!(
        "relation ready in {:?}: {:?}",
        start_time.elapsed(),
        graph.size()
    );

    let mapping = graph.export_file_issue_mapping();

    // to stdout
    let json_string = serde_json::to_string(&mapping).expect("Failed to serialize to JSON");
    print!("{}", json_string);
}

fn handle_up(up_cmd: UpCommand) {
    tracing_subscriber::fmt::init();

    info!("relation creating ...");
    let mut conf = Config::default();
    if let Some(ref user_issue_regex) = up_cmd.common_options.issue_regex {
        conf.issue_regex = user_issue_regex.clone()
    }
    if let Some(ref repo_path) = up_cmd.common_options.repo_path {
        conf.repo_path = repo_path.clone()
    }
    if let Some(ref path_specs) = up_cmd.common_options.path_specs {
        conf.path_specs = path_specs.split(";").map(|a| a.into()).collect();
    }
    if let Some(ref multi_parents) = up_cmd.common_options.multi_parents {
        conf.multi_parents = multi_parents.clone()
    }
    if let Some(ref progress) = up_cmd.common_options.progress {
        conf.progress = progress.clone()
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

    let mut server_conf = ServerConfig::new(graph);
    if let Some(ref port) = up_cmd.port {
        server_conf.port = *port
    }
    info!("server up: http://127.0.0.1:{}", server_conf.port);
    server_main(server_conf);
}
