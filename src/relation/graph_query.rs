use crate::relation::graph::{GraphSize, NodeData, NodeMapping, RelationGraph};
use std::fmt::Error;

/// query API
impl RelationGraph {
    pub fn get_file_node(&self, name: &String) -> Option<&NodeData> {
        if !self.file_mapping.contains_key(name) {
            return None;
        }
        let node_index = self.file_mapping.get(name).unwrap();
        return Some(&self.g[*node_index]);
    }

    pub fn get_commit_node(&self, name: &String) -> Option<&NodeData> {
        if !self.commit_mapping.contains_key(name) {
            return None;
        }
        let node_index = self.commit_mapping.get(name).unwrap();
        return Some(&self.g[*node_index]);
    }

    pub fn get_issue_node(&self, name: &String) -> Option<&NodeData> {
        if !self.issue_mapping.contains_key(name) {
            return None;
        }
        let node_index = self.issue_mapping.get(name).unwrap();
        return Some(&self.g[*node_index]);
    }

    pub(crate) fn get_keys(&self, node_mapping: &NodeMapping) -> Vec<String> {
        return node_mapping
            .keys()
            .map(|key| key.as_ref().clone())
            .collect();
    }

    pub fn files(&self) -> Vec<String> {
        return self.get_keys(&self.file_mapping);
    }

    pub fn commits(&self) -> Vec<String> {
        return self.get_keys(&self.commit_mapping);
    }

    pub fn issues(&self) -> Vec<String> {
        return self.get_keys(&self.issue_mapping);
    }

    pub(crate) fn find_related(
        &self,
        entry: &String,
        src: &NodeMapping,
        target: &NodeMapping,
    ) -> Result<Vec<String>, Error> {
        if !src.contains_key(entry) {
            return Err(Error::default());
        }
        let related: Vec<String> = self
            .g
            .neighbors(src[entry])
            .filter(|node_index| {
                let data = &self.g[*node_index];
                return target.contains_key(&data.name);
            })
            .map(|node_index| {
                return self.g[node_index].name.to_string().clone();
            })
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
