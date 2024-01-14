use std::fs::File;
use std::io::Write;
use git2::{Commit, Repository};
use petgraph::graph::DiGraph;


fn main() {
    let conf = Config::new(".", 1024);
    walk(conf);
}

fn walk(conf: Config) {
    let repo_path = conf.repo_path;

    let repo = Repository::open(repo_path).expect("Failed to open repository");
    let head = repo.head().expect("Failed to get HEAD ref").peel_to_commit().expect("Failed to peel HEAD to commit");
    let mut revwalk = repo.revwalk().expect("Failed to create revwalk");
    revwalk.push(head.id()).expect("Failed to push commit");
    let _ = revwalk.set_sorting(git2::Sort::TIME | git2::Sort::REVERSE);

    let mut counter = 0;
    let mut graph = DiGraph::<String, i32>::new();


    for id in revwalk {
        if let Ok(commit_id) = id {
            if let Ok(commit) = repo.find_commit(commit_id) {
                let commit_files = process_commit(&repo, &commit);
                let commit_node = graph.add_node(commit_id.to_string());

                for file in commit_files {
                    let file_node = graph.add_node(file.clone());
                    graph.add_edge(commit_node, file_node, 1);
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

    let dot = petgraph::dot::Dot::with_config(&graph, &[]);
    if let Ok(mut file) = File::create("graph.dot") {
        file.write_all(dot.to_string().as_bytes()).expect("Failed to write to file");
        println!("DOT representation saved to 'graph.dot'");
    } else {
        eprintln!("Failed to create or write to 'graph.dot'");
    }
    println!("finished");
}

fn process_commit(repo: &Repository, commit: &Commit) -> Vec<String> {
    if let Some(parent) = commit.parent(0).ok() {
        let parent_tree = parent.tree().expect("Failed to get parent tree");
        let current_tree = commit.tree().expect("Failed to get commit tree");

        // Compare the trees and print changed files
        let changes = repo.diff_tree_to_tree(Some(&parent_tree), Some(&current_tree), None).expect("Failed to get diff");
        let changed_files: Vec<String> = changes
            .deltas()
            .filter_map(|delta| delta.new_file().path().map(|path| path.to_string_lossy().into_owned()))
            .collect();

        return changed_files;
    }
    return Vec::new();
}

struct Config {
    repo_path: String,
    depth: i32,
}

impl Config {
    fn new(repo_path: &str, depth: i32) -> Config {
        Config {
            repo_path: repo_path.to_string(),
            depth,
        }
    }
}
