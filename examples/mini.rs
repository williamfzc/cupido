use cupido::collector::config::{Config, get_collector, Collect};

fn main() {
    let collector = get_collector();
    let graph = collector.walk(Config::default());

    // 1. search from files to issues
    let file_name = String::from("src/server/app.rs");
    let issues = graph.file_related_issues(&file_name).unwrap();

    // src/server/app.rs related to ["#1"]
    println!("1. {} related to {:?}", file_name, issues);

    // 2. search from issues to commits
    let issue_label = issues.get(0).unwrap();
    let commits = graph.issue_related_commits(issue_label).unwrap();

    // #1 related to ["b7574411fbf685a777d1929bff26b3ad4ebd84f2"]
    println!("2. {} related to {:?}", issue_label, commits);

    // 3. search from commits to files
    let commit = commits.get(0).unwrap();
    let files = graph.commit_related_files(commit).unwrap();

    // b7574411fbf685a777d1929bff26b3ad4ebd84f2 related to ["src/server/mod.rs", "src/server/handler.rs", "src/server/config.rs", "src/server/app.rs", "src/server.rs", "src/main.rs"]
    println!("3. {} related to {:?}", commit, files);

    // Also, you can do it vice versa.
}
