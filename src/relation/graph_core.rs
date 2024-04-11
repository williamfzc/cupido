use crate::relation::graph::{EdgeType, NodeData, NodeMapping, RelationGraph};
use petgraph::graph::{NodeIndex, UnGraph};
use std::sync::Arc;

/// core functions for generating graph
impl RelationGraph {
    pub fn new() -> RelationGraph {
        return RelationGraph {
            file_mapping: NodeMapping::new(),
            commit_mapping: NodeMapping::new(),
            issue_mapping: NodeMapping::new(),
            author_mapping: NodeMapping::new(),
            g: UnGraph::<NodeData, EdgeType>::new_undirected(),
            conf: crate::collector::config::Config::default(),
        };
    }

    pub(crate) fn add_node(&mut self, name: &String, node_type: crate::relation::graph::NodeType) {
        let mapping = match node_type {
            crate::relation::graph::NodeType::Commit(_) => &mut self.commit_mapping,
            crate::relation::graph::NodeType::File(_) => &mut self.file_mapping,
            crate::relation::graph::NodeType::Issue(_) => &mut self.issue_mapping,
            crate::relation::graph::NodeType::Author(_) => &mut self.author_mapping,
        };

        if !mapping.contains_key(name) {
            let name_rc: Arc<String> = Arc::from(name.to_string());
            let node_data = NodeData {
                name: name_rc.clone(),
                _node_type: node_type,
            };
            let node_index = self.g.add_node(node_data);
            mapping.insert(name_rc, node_index);
        }
    }

    pub fn add_commit_node(&mut self, name: &String) {
        return self.add_node(name, crate::relation::graph::NodeType::Commit(None));
    }

    pub fn add_file_node(&mut self, name: &String) {
        return self.add_node(name, crate::relation::graph::NodeType::File(None));
    }

    pub fn add_issue_node(&mut self, name: &String) {
        return self.add_node(name, crate::relation::graph::NodeType::Issue(None));
    }

    pub(crate) fn add_edge(
        &mut self,
        source_index: NodeIndex,
        target_index: NodeIndex,
        edge_type: EdgeType,
    ) {
        if let Some(..) = self.g.find_edge(source_index, target_index) {
            return;
        }
        self.g.add_edge(source_index, target_index, edge_type);
    }

    pub fn add_edge_file2commit(&mut self, file_name: &String, commit_name: &String) {
        if let (Some(file_index), Some(commit_index)) = (
            self.file_mapping.get(file_name),
            self.commit_mapping.get(commit_name),
        ) {
            self.add_edge(*file_index, *commit_index, EdgeType::File2Commit);
        }
    }

    pub fn add_edge_file2issue(&mut self, file_name: &String, issue: &String) {
        if let (Some(file_index), Some(issue_index)) = (
            self.file_mapping.get(file_name),
            self.issue_mapping.get(issue),
        ) {
            self.add_edge(*file_index, *issue_index, EdgeType::File2Issue);
        }
    }

    pub fn add_edge_commit2issue(&mut self, commit_name: &String, issue: &String) {
        if let (Some(commit_index), Some(issue_index)) = (
            self.commit_mapping.get(commit_name),
            self.issue_mapping.get(issue),
        ) {
            self.add_edge(*commit_index, *issue_index, EdgeType::Commit2Issue);
        }
    }
}
