use crate::relation::graph::{EdgeType, NodeType, RelationGraph};

/// extension functions
impl RelationGraph {
    pub fn add_author_node(&mut self, name: &String) {
        return self.add_node(name, NodeType::Author(None));
    }

    pub fn add_edge_author2commit(&mut self, author_name: &String, commit_name: &String) {
        if let (Some(commit_data), Some(author_data)) = (
            self.commit_mapping.get(commit_name),
            self.author_mapping.get(author_name),
        ) {
            let commit_index = commit_data.node_index;
            let author_index = author_data.node_index;
            self.add_edge(commit_index, author_index, EdgeType::Author2Commit);
        }
    }
}
