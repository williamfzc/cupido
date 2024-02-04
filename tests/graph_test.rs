use cupido::collector::config::Collect;
use cupido::collector::config::{get_collector, Config};

#[test]
fn graph_query() {
    // Collect the graph
    let collector = get_collector();
    let graph = collector.walk(Config::default());

    // Test file-related issues
    let file_name = String::from("src/server/app.rs");
    let issues_result = graph.file_related_issues(&file_name);
    assert!(
        issues_result.is_ok(),
        "File-related issues query should succeed"
    );

    let issues = issues_result.unwrap_or_default();
    assert!(
        !issues.is_empty(),
        "File should be related to at least one issue"
    );

    // Test issue-related files
    let issue_name = &issues[0];
    let files_result = graph.issue_related_files(issue_name);
    assert!(
        files_result.is_ok(),
        "Issue-related files query should succeed"
    );

    let files = files_result.unwrap_or_default();
    assert!(
        !files.is_empty(),
        "Issue should be related to at least one file"
    );

    // Print results for inspection
    println!("File: {}", file_name);
    println!("File-related issues: {:?}", issues);
    println!("Issue: {}", issue_name);
    println!("Issue-related files: {:?}", files);
}