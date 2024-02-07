use petgraph::dot::Config;
use petgraph::graph::{NodeIndex, UnGraph};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use std::fs::File;
use std::io::Write;
use std::sync::Arc;

enum NodeType {
    File(Option<FileData>),
    Commit(Option<CommitData>),
    Issue(Option<IssueData>),
}

struct FileData {}

struct CommitData {}

struct IssueData {}

#[derive(Debug)]
enum EdgeType {
    File2Commit,
    File2Issue,
    Commit2Issue,
}

impl Display for EdgeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct NodeData {
    _name: Arc<String>,
    _node_type: NodeType,
    node_index: NodeIndex,
}

type NodeMapping = HashMap<Arc<String>, NodeData>;

pub struct RelationGraph {
    file_mapping: NodeMapping,
    commit_mapping: NodeMapping,
    issue_mapping: NodeMapping,
    author_mapping: NodeMapping,
    g: UnGraph<Arc<String>, EdgeType>,
}

// core functions
impl RelationGraph {
    pub fn new() -> RelationGraph {
        return RelationGraph {
            file_mapping: NodeMapping::new(),
            commit_mapping: NodeMapping::new(),
            issue_mapping: NodeMapping::new(),
            author_mapping: NodeMapping::new(),
            g: UnGraph::<Arc<String>, EdgeType>::new_undirected(),
        };
    }

    fn add_node(&mut self, name: &String, node_type: NodeType) {
        let mapping = match node_type {
            NodeType::Commit(_) => &mut self.commit_mapping,
            NodeType::File(_) => &mut self.file_mapping,
            NodeType::Issue(_) => &mut self.issue_mapping,
        };

        if !mapping.contains_key(name) {
            let name_rc: Arc<String> = Arc::from(name.to_string());
            let node_index = self.g.add_node(name_rc.clone());
            let node_data = NodeData {
                _name: name_rc.clone(),
                _node_type: node_type,
                node_index,
            };
            mapping.insert(name_rc, node_data);
        }
    }

    pub fn add_commit_node(&mut self, name: &String) {
        return self.add_node(name, NodeType::Commit(None));
    }

    pub fn add_file_node(&mut self, name: &String) {
        return self.add_node(name, NodeType::File(None));
    }

    pub fn add_issue_node(&mut self, name: &String) {
        return self.add_node(name, NodeType::Issue(None));
    }

    pub fn get_file_node(&self, name: &String) -> Option<&NodeData> {
        self.file_mapping.get(name)
    }

    pub fn get_commit_node(&self, name: &String) -> Option<&NodeData> {
        self.commit_mapping.get(name)
    }

    pub fn get_issue_node(&self, name: &String) -> Option<&NodeData> {
        self.issue_mapping.get(name)
    }

    fn add_edge(&mut self, source_index: NodeIndex, target_index: NodeIndex, edge_type: EdgeType) {
        if let Some(..) = self.g.find_edge(source_index, target_index) {
            return;
        }
        self.g.add_edge(source_index, target_index, edge_type);
    }

    pub fn add_edge_file2commit(&mut self, file_name: &String, commit_name: &String) {
        if let (Some(file_data), Some(commit_data)) = (
            self.file_mapping.get(file_name),
            self.commit_mapping.get(commit_name),
        ) {
            let file_index = file_data.node_index;
            let commit_index = commit_data.node_index;
            self.add_edge(file_index, commit_index, EdgeType::File2Commit);
        }
    }

    pub fn add_edge_file2issue(&mut self, file_name: &String, issue: &String) {
        if let (Some(file_data), Some(issue_data)) = (
            self.file_mapping.get(file_name),
            self.issue_mapping.get(issue),
        ) {
            let file_index = file_data.node_index;
            let issue_index = issue_data.node_index;
            self.add_edge(file_index, issue_index, EdgeType::File2Issue);
        }
    }

    pub fn add_edge_commit2issue(&mut self, commit_name: &String, issue: &String) {
        if let (Some(commit_data), Some(issue_data)) = (
            self.commit_mapping.get(commit_name),
            self.issue_mapping.get(issue),
        ) {
            let commit_index = commit_data.node_index;
            let issue_index = issue_data.node_index;
            self.add_edge(commit_index, issue_index, EdgeType::Commit2Issue);
        }
    }

    fn find_related(
        &self,
        entry: &String,
        src: &NodeMapping,
        target: &NodeMapping,
    ) -> Result<Vec<String>, Error> {
        if !src.contains_key(entry) {
            return Err(Error::default());
        }
        let neighbors = self.g.neighbors(src[entry].node_index);
        let related: Vec<String> = neighbors
            .filter(|node_index| {
                let data = self.g[*node_index].to_string();
                if !target.contains_key(&data) {
                    return false;
                }
                return true;
            })
            .map(|node_index| self.g[node_index].to_string())
            .collect();

        Ok(related)
    }

    pub fn file_related_commits(&self, file_name: &String) -> Result<Vec<String>, Error> {
        return self.find_related(file_name, &self.file_mapping, &self.commit_mapping);
    }

    pub fn file_related_issues(&self, file_name: &String) -> Result<Vec<String>, Error> {
        return self.find_related(file_name, &self.file_mapping, &self.issue_mapping);
    }

    pub fn issue_related_files(&self, issue_name: &String) -> Result<Vec<String>, Error> {
        return self.find_related(issue_name, &self.issue_mapping, &self.file_mapping);
    }

    pub fn issue_related_commits(&self, issue_name: &String) -> Result<Vec<String>, Error> {
        return self.find_related(issue_name, &self.issue_mapping, &self.commit_mapping);
    }

    pub fn commit_related_files(&self, commit_name: &String) -> Result<Vec<String>, Error> {
        return self.find_related(commit_name, &self.commit_mapping, &self.file_mapping);
    }

    pub fn commit_related_issues(&self, commit_name: &String) -> Result<Vec<String>, Error> {
        return self.find_related(commit_name, &self.commit_mapping, &self.issue_mapping);
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

// extension functions
impl RelationGraph {}

// export
impl RelationGraph {
    pub fn export_dot(&self, file_path: &str) {
        // copy a new graph for filters
        let mut graph = RelationGraph::new();
        for (each, _) in &self.file_mapping {
            graph.add_file_node(each)
        }
        for (each, _) in &self.issue_mapping {
            graph.add_issue_node(each);
            for each_file in &self.issue_related_files(each).unwrap() {
                graph.add_edge_file2issue(each_file, each)
            }
        }

        let dot = petgraph::dot::Dot::with_config(&graph.g, &[Config::EdgeNoLabel]);
        if let Ok(mut file) = File::create(file_path) {
            file.write_all(dot.to_string().as_bytes())
                .expect("Failed to write to file");
            println!("DOT representation saved to '{}'", file_path);
        } else {
            eprintln!("Failed to create or write to '{}'", file_path);
        }
    }

    pub fn export_file_issue_mapping(&self) -> HashMap<String, Vec<String>> {
        let mut ret = HashMap::new();
        for (f, _) in &self.file_mapping {
            let fs = f.to_string();
            let issues: Result<Vec<String>, Error> = self.file_related_issues(&fs);

            if let Ok(ok_issues) = issues {
                if !ok_issues.is_empty() {
                    ret.insert(fs.clone(), ok_issues);
                }
            }
        }
        return ret;
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GraphSize {
    file_size: usize,
    commit_size: usize,
    issue_size: usize,
}
