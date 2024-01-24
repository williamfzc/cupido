use crate::graph::RelationGraph;
use git2::{Commit, Repository};
use regex::Regex;

pub fn walk(conf: Config) -> RelationGraph {
    let repo_path = conf.repo_path;

    let repo = Repository::open(repo_path).expect("Failed to open repository");
    let head = repo
        .head()
        .expect("Failed to get HEAD ref")
        .peel_to_commit()
        .expect("Failed to peel HEAD to commit");
    let mut revwalk = repo.revwalk().expect("Failed to create revwalk");
    revwalk.push(head.id()).expect("Failed to push commit");

    // top to bottom
    let _ = revwalk.set_sorting(git2::Sort::TIME);

    let mut counter = 0;
    let mut graph = RelationGraph::new();

    let issue_regex: Regex = Regex::new(&*conf.issue_regex).unwrap();

    for id in revwalk {
        if let Ok(commit_id) = id {
            if let Ok(commit) = repo.find_commit(commit_id) {
                let commit_result = process_commit(&repo, &commit, &issue_regex);

                // files
                for file in &commit_result.files {
                    graph.add_file_node(file);
                }
                // commits
                let commit_id_str = &commit_id.to_string();
                graph.add_commit_node(commit_id_str);
                for file in &commit_result.files {
                    graph.add_edge_file2commit(file, commit_id_str, &String::new());
                }
                // issues
                for issue in &commit_result.issues {
                    graph.add_issue_node(issue);

                    for file in &commit_result.files {
                        graph.add_edge_file2issue(file, issue, &String::new());
                    }
                }

                counter += 1;
                if counter > conf.depth {
                    break;
                }
            } else {
                eprintln!("Failed to find commit {}", commit_id);
            }
        } else {
            eprintln!("Failed to get commit id");
        }
    }

    return graph;
}

fn process_commit(repo: &Repository, commit: &Commit, re: &Regex) -> CommitResult {
    if let Some(parent) = commit.parent(0).ok() {
        let parent_tree = parent.tree().expect("Failed to get parent tree");
        let current_tree = commit.tree().expect("Failed to get commit tree");

        // Compare the trees and print changed files
        let changes = repo
            .diff_tree_to_tree(Some(&parent_tree), Some(&current_tree), None)
            .expect("Failed to get diff");
        let changed_files: Vec<String> = changes
            .deltas()
            .filter_map(|delta| {
                delta
                    .new_file()
                    .path()
                    .map(|path| path.to_string_lossy().into_owned())
            })
            .collect();

        // Issue extract
        let issues = re
            .find_iter(commit.message().unwrap_or_default())
            .map(|mat| mat.as_str().to_string())
            .collect();

        return CommitResult {
            files: changed_files,
            issues,
        };
    }
    return CommitResult::default();
}

struct CommitResult {
    files: Vec<String>,
    issues: Vec<String>,
}

impl CommitResult {
    pub fn default() -> CommitResult {
        return CommitResult {
            files: Vec::new(),
            issues: Vec::new(),
        };
    }
}

pub struct Config {
    pub repo_path: String,
    pub depth: i32,
    pub issue_regex: String,
}

impl Config {
    pub fn default() -> Config {
        return Config {
            repo_path: String::from("."),
            depth: 10240,
            issue_regex: String::from(r"(#\d+)"),
        };
    }
}
