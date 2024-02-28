use crate::collector::native::NativeCollector;
use crate::relation::graph::RelationGraph;
use serde_derive::{Deserialize, Serialize};

pub struct CommitResult {
    pub files: Vec<String>,
    pub issues: Vec<String>,
}

impl CommitResult {
    pub fn default() -> CommitResult {
        return CommitResult {
            files: Vec::new(),
            issues: Vec::new(),
        };
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub repo_path: String,
    pub depth: i32,
    pub issue_regex: String,
    pub path_specs: Vec<String>,
    pub multi_parents: bool,
    pub progress: bool,
    // todo: node types should be optional
}

impl Clone for Config {
    fn clone(&self) -> Self {
        Self {
            repo_path: self.repo_path.clone(),
            depth: self.depth,
            issue_regex: self.issue_regex.clone(),
            path_specs: self.path_specs.clone(),
            multi_parents: self.multi_parents,
            progress: self.progress,
        }
    }
}

impl Config {
    pub fn default() -> Config {
        return Config {
            repo_path: String::from("."),
            depth: 10240,
            issue_regex: String::from(r"(#\d+)"),
            path_specs: Vec::default(),
            multi_parents: false,
            progress: false,
        };
    }
}

pub trait Collect {
    fn walk(&self, conf: Config) -> RelationGraph;
}

pub fn get_collector() -> impl Collect {
    let collector = NativeCollector {};
    return collector;
}
