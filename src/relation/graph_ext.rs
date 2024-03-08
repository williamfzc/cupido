use crate::relation::graph::{EdgeType, NodeData, NodeType, RelationGraph};
use std::fmt::Error;

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

    pub fn get_author_node(&self, name: &String) -> Option<&NodeData> {
        return self.author_mapping.get(name);
    }

    pub fn author_related_commits(&self, author_name: &String) -> Result<Vec<String>, Error> {
        return self.find_related(author_name, &self.author_mapping, &self.commit_mapping);
    }

    pub fn commit_related_authors(&self, commit_name: &String) -> Result<Vec<String>, Error> {
        // why still using author(s) not author:
        // https://docs.github.com/en/pull-requests/committing-changes-to-your-project/creating-and-editing-commits/creating-a-commit-with-multiple-authors
        // parse co-author in the future
        return self.find_related(commit_name, &self.commit_mapping, &self.author_mapping);
    }

    pub fn authors(&self) -> Result<Vec<String>, Error> {
        let keys: Vec<String> = self
            .author_mapping
            .keys()
            .map(|key| key.as_ref().clone())
            .collect();
        return Ok(keys);
    }
}
