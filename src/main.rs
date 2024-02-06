use clap::Parser;
use cupido::collector::config::get_collector;
use cupido::collector::config::Collect;
use cupido::collector::config::Config;
use cupido::server::app::server_main;
use cupido::server::config::ServerConfig;
use std::fs::File;
use std::io::Write;
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

    /// Extract file-issue mapping
    Map(MapCommand),
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

    /// Multi parents search
    #[clap(short, long)]
    multi_parents: Option<bool>,
}

#[derive(Parser, Debug)]
struct MapCommand {
    /// For catching issues in commit message
    #[clap(short, long)]
    issue_regex: Option<String>,

    /// Root location
    #[clap(short, long)]
    repo_path: Option<String>,

    /// File include
    #[clap(short, long)]
    path_specs: Option<String>,

    /// Output
    #[clap(short, long)]
    output_json: Option<String>,

    /// Multi parents search
    #[clap(short, long)]
    multi_parents: Option<bool>,
}

fn main() {
    let cli: Cli = Cli::parse();

    match cli.cmd {
        SubCommand::Up(up_cmd) => handle_up(up_cmd),
        SubCommand::Map(map_cmd) => handle_map(map_cmd),
    }
}

fn handle_map(map_command: MapCommand) {
    tracing_subscriber::fmt::init();

    info!("relation creating ...");
    let mut conf = Config::default();
    if let Some(ref user_issue_regex) = map_command.issue_regex {
        conf.issue_regex = user_issue_regex.clone()
    }
    if let Some(ref repo_path) = map_command.repo_path {
        conf.repo_path = repo_path.clone()
    }
    if let Some(ref path_specs) = map_command.path_specs {
        conf.path_specs = path_specs.split(";").map(|a| a.into()).collect();
    }
    if let Some(ref multi_parents) = map_command.multi_parents {
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

    // to fs
    let json_string = serde_json::to_string(&mapping).expect("Failed to serialize to JSON");
    let file_path = map_command.output_json.unwrap_or("output.json".to_string());
    let mut file = File::create(file_path).expect("Failed to create file");
    file.write_all(json_string.as_bytes())
        .expect("Failed to write to file");
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
    if let Some(ref multi_parents) = up_cmd.multi_parents {
        conf.multi_parents = multi_parents.clone()
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
