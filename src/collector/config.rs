use crate::collector::native::NativeCollector;
use crate::relation::graph::RelationGraph;
use serde_derive::{Deserialize, Serialize};

pub struct CommitResult {
    pub files: Vec<String>,
    pub issues: Vec<String>,
}

impl CommitResult {
    pub fn default() -> CommitResult {
        CommitResult {
            files: Vec::new(),
            issues: Vec::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub repo_path: String,
    pub depth: u32,
    pub issue_regex: String,
    pub path_specs: Vec<String>,
    pub multi_parents: bool,
    pub progress: bool,
    pub commit_exclude_regex: Option<String>,
    pub author_exclude_regex: Option<String>,
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
            commit_exclude_regex: self.commit_exclude_regex.clone(),
            author_exclude_regex: self.author_exclude_regex.clone(),
        }
    }
}

impl Config {
    pub fn default() -> Config {
        Config {
            repo_path: String::from("."),
            depth: 10240,
            issue_regex: String::from(r"(#\d+)"),
            path_specs: Vec::default(),
            multi_parents: false,
            progress: false,
            commit_exclude_regex: None,
            author_exclude_regex: None,
        }
    }
}

pub trait Collect {
    fn walk(&self, conf: Config) -> RelationGraph;
}

pub fn get_collector() -> impl Collect {
    let collector = NativeCollector {};
    collector
}
