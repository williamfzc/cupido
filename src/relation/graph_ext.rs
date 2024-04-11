use crate::relation::graph::{EdgeType, NodeData, NodeType, RelationGraph};
use std::collections::HashMap;
use std::fmt::Error;

/// extension functions
impl RelationGraph {
    pub fn add_author_node(&mut self, name: &String) {
        return self.add_node(name, NodeType::Author(None));
    }

    pub fn add_edge_author2commit(&mut self, author_name: &String, commit_name: &String) {
        if let (Some(commit_index), Some(author_index)) = (
            self.commit_mapping.get(commit_name),
            self.author_mapping.get(author_name),
        ) {
            self.add_edge(*commit_index, *author_index, EdgeType::Author2Commit);
        }
    }

    pub fn get_author_node(&self, name: &String) -> Option<&NodeData> {
        if !self.author_mapping.contains_key(name) {
            return None;
        }
        let node_index = self.author_mapping.get(name).unwrap();
        return Some(&self.g[*node_index]);
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

    pub fn authors(&self) -> Vec<String> {
        return self.get_keys(&self.author_mapping);
    }

    fn file_edge_counter(&self) -> HashMap<String, usize> {
        let mut edges_count_map: HashMap<_, usize> = HashMap::new();
        for (each_name, each) in &self.file_mapping {
            let edges = self.g.edges(*each);
            let edge_count = edges.count();
            edges_count_map.insert(each_name.to_string(), edge_count);
        }
        return edges_count_map;
    }

    pub fn file_hot_ranks(&self) -> HashMap<String, usize> {
        // currently we directly simply use edge counts for representing heat rates
        // commits + issues
        let edges_count_map = self.file_edge_counter();
        let mut sorted_edges_count: Vec<_> = edges_count_map.into_iter().collect();
        sorted_edges_count.sort_by(|a, b| a.1.cmp(&b.1));

        // same scores might have different ranks
        let mut ranks: HashMap<String, usize> = HashMap::new();
        for (idx, (file_name, _)) in sorted_edges_count.iter().enumerate() {
            ranks.insert(file_name.clone(), idx + 1);
        }
        return ranks;
    }

    pub fn file_hot_rank(&self, file_name: &String) -> usize {
        const DEFAULT: usize = 0;
        if !self.file_mapping.contains_key(file_name) {
            return DEFAULT;
        }
        let ranks = self.file_hot_ranks();
        return *ranks.get(file_name).unwrap_or(&DEFAULT);
    }
}
