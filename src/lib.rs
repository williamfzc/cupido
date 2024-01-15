use std::collections::HashMap;
use std::fmt::Error;
use std::fs::File;
use std::io::Write;
use git2::{Commit, Repository};
use petgraph::graph::{NodeIndex, UnGraph};

pub fn walk(conf: Config) -> CupidGraph {
    let repo_path = conf.repo_path;

    let repo = Repository::open(repo_path).expect("Failed to open repository");
    let head = repo.head().expect("Failed to get HEAD ref").peel_to_commit().expect("Failed to peel HEAD to commit");
    let mut revwalk = repo.revwalk().expect("Failed to create revwalk");
    revwalk.push(head.id()).expect("Failed to push commit");
    let _ = revwalk.set_sorting(git2::Sort::TIME | git2::Sort::REVERSE);

    let mut counter = 0;
    let mut graph = CupidGraph::new();

    for id in revwalk {
        if let Ok(commit_id) = id {
            if let Ok(commit) = repo.find_commit(commit_id) {
                let commit_files = process_commit(&repo, &commit);

                graph.add_commit_node(commit_id.to_string());
                for file in commit_files {
                    graph.add_file_node(file.clone());
                    graph.add_edge(file, commit_id.to_string(), String::new());
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

pub struct Config {
    repo_path: String,
    depth: i32,
}

impl Config {
    pub fn new(repo_path: &str, depth: i32) -> Config {
        Config {
            repo_path: repo_path.to_string(),
            depth,
        }
    }
}

enum NodeType {
    File,
    Commit,
}

pub struct NodeData {
    name: String,
    node_type: NodeType,
    node_index: NodeIndex,
}

pub struct CupidGraph {
    file_mapping: HashMap<String, NodeData>,
    commit_mapping: HashMap<String, NodeData>,
    g: UnGraph<String, String>,
}

impl CupidGraph {
    pub fn new() -> CupidGraph {
        return CupidGraph {
            file_mapping: HashMap::<String, NodeData>::new(),
            commit_mapping: HashMap::<String, NodeData>::new(),
            g: UnGraph::<String, String>::new_undirected(),
        };
    }

    pub fn add_commit_node(&mut self, name: String) {
        if !self.commit_mapping.contains_key(&name) {
            let node_index = self.g.add_node(name.clone());
            let node_data = NodeData {
                name: name.clone(),
                node_type: NodeType::Commit,
                node_index,
            };
            self.commit_mapping.insert(name, node_data);
        }
    }

    pub fn add_file_node(&mut self, name: String) {
        if !self.file_mapping.contains_key(&name) {
            let node_index = self.g.add_node(name.clone());
            let node_data = NodeData {
                name: name.clone(),
                node_type: NodeType::File,
                node_index,
            };
            self.file_mapping.insert(name, node_data);
        }
    }

    pub fn get_file_node(&self, name: String) -> Option<&NodeData> {
        self.file_mapping.get(&name)
    }

    pub fn get_commit_node(&self, name: String) -> Option<&NodeData> {
        self.commit_mapping.get(&name)
    }

    pub fn add_edge(&mut self, file_name: String, commit_name: String, edge_label: String) {
        if let (Some(file_data), Some(commit_data)) = (self.file_mapping.get(&file_name), self.commit_mapping.get(&commit_name)) {
            let file_index = file_data.node_index;
            let commit_index = commit_data.node_index;
            self.g.add_edge(file_index, commit_index, edge_label.to_string());
        }
    }

    pub fn related_commits(self, file_name: String) -> Result<Vec<String>, Error> {
        if !self.file_mapping.contains_key(&file_name) {
            return Err(Error::default());
        }
        let neighbors = self.g.neighbors(self.get_file_node(file_name).unwrap().node_index);
        let related_commits: Vec<String> = neighbors
            .map(|node_data| self.g[node_data].clone())
            .collect();

        Ok(related_commits)
    }

    pub fn export_dot(&self, file_path: &str) {
        let dot = petgraph::dot::Dot::with_config(&self.g, &[]);
        if let Ok(mut file) = File::create(file_path) {
            file.write_all(dot.to_string().as_bytes()).expect("Failed to write to file");
            println!("DOT representation saved to 'graph.dot'");
        } else {
            eprintln!("Failed to create or write to 'graph.dot'");
        }
    }
}