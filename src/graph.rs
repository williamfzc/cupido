use petgraph::graph::{NodeIndex, UnGraph};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Error;
use std::fs::File;
use std::io::Write;

enum NodeType {
    File,
    Commit,
    Issue,
}

pub struct NodeData {
    _name: String,
    _node_type: NodeType,
    node_index: NodeIndex,
}

pub struct RelationGraph {
    file_mapping: HashMap<String, NodeData>,
    commit_mapping: HashMap<String, NodeData>,
    issue_mapping: HashMap<String, NodeData>,
    g: UnGraph<String, String>,
}

impl RelationGraph {
    pub fn new() -> RelationGraph {
        return RelationGraph {
            file_mapping: HashMap::<String, NodeData>::new(),
            commit_mapping: HashMap::<String, NodeData>::new(),
            issue_mapping: HashMap::<String, NodeData>::new(),
            g: UnGraph::<String, String>::new_undirected(),
        };
    }

    pub fn add_commit_node(&mut self, name: &String) {
        if !self.commit_mapping.contains_key(name) {
            let node_index = self.g.add_node(name.clone());
            let node_data = NodeData {
                _name: name.clone(),
                _node_type: NodeType::Commit,
                node_index,
            };
            self.commit_mapping.insert(name.to_string(), node_data);
        }
    }

    pub fn add_file_node(&mut self, name: &String) {
        if !self.file_mapping.contains_key(name) {
            let node_index = self.g.add_node(name.clone());
            let node_data = NodeData {
                _name: name.clone(),
                _node_type: NodeType::File,
                node_index,
            };
            self.file_mapping.insert(name.to_string(), node_data);
        }
    }

    pub fn add_issue_node(&mut self, name: &String) {
        if !self.issue_mapping.contains_key(name) {
            let node_index = self.g.add_node(name.clone());
            let node_data = NodeData {
                _name: name.clone(),
                _node_type: NodeType::Issue,
                node_index,
            };
            self.issue_mapping.insert(name.to_string(), node_data);
        }
    }

    pub fn get_file_node(&self, name: &String) -> Option<&NodeData> {
        self.file_mapping.get(name)
    }

    pub fn get_commit_node(&self, name: &String) -> Option<&NodeData> {
        self.commit_mapping.get(name)
    }

    pub fn add_edge_file2commit(
        &mut self,
        file_name: &String,
        commit_name: &String,
        edge_label: &String,
    ) {
        if let (Some(file_data), Some(commit_data)) = (
            self.file_mapping.get(file_name),
            self.commit_mapping.get(commit_name),
        ) {
            let file_index = file_data.node_index;
            let commit_index = commit_data.node_index;
            self.g
                .add_edge(file_index, commit_index, edge_label.to_string());
        }
    }

    pub fn add_edge_file2issue(&mut self, file_name: &String, issue: &String, edge_label: &String) {
        if let (Some(file_data), Some(issue_data)) = (
            self.file_mapping.get(file_name),
            self.issue_mapping.get(issue),
        ) {
            let file_index = file_data.node_index;
            let commit_index = issue_data.node_index;
            self.g
                .add_edge(file_index, commit_index, edge_label.to_string());
        }
    }

    pub fn related_commits(&self, file_name: &String) -> Result<Vec<String>, Error> {
        if !self.file_mapping.contains_key(file_name) {
            return Err(Error::default());
        }
        let neighbors = self
            .g
            .neighbors(self.get_file_node(file_name).unwrap().node_index);
        let related_commits: Vec<String> = neighbors
            .filter(|node_index| {
                let data = self.g[*node_index].to_string();
                if !self.commit_mapping.contains_key(&data) {
                    return false;
                }
                return true;
            })
            .map(|node_index| self.g[node_index].clone())
            .collect();

        Ok(related_commits)
    }

    pub fn related_issues(&self, file_name: &String) -> Result<Vec<String>, Error> {
        if !self.file_mapping.contains_key(file_name) {
            return Err(Error::default());
        }
        let neighbors = self
            .g
            .neighbors(self.get_file_node(file_name).unwrap().node_index);
        let related_issues: Vec<String> = neighbors
            .filter(|node_index| {
                let data = self.g[*node_index].to_string();
                if !self.issue_mapping.contains_key(&data) {
                    return false;
                }
                return true;
            })
            .map(|node_index| self.g[node_index].clone())
            .collect();

        Ok(related_issues)
    }

    pub fn export_dot(&self, file_path: &str) {
        let dot = petgraph::dot::Dot::with_config(&self.g, &[]);
        if let Ok(mut file) = File::create(file_path) {
            file.write_all(dot.to_string().as_bytes())
                .expect("Failed to write to file");
            println!("DOT representation saved to 'graph.dot'");
        } else {
            eprintln!("Failed to create or write to 'graph.dot'");
        }
    }

    pub fn file_size(&self) -> usize {
        return self.file_mapping.len();
    }

    pub fn commit_size(&self) -> usize {
        return self.commit_mapping.len();
    }

    pub fn issue_size(&self) -> usize {
        return self.issue_mapping.len();
    }

    pub fn size(&self) -> GraphSize {
        return GraphSize {
            file_size: self.file_size(),
            commit_size: self.commit_size(),
            issue_size: self.issue_size(),
        };
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GraphSize {
    file_size: usize,
    commit_size: usize,
    issue_size: usize,
}
