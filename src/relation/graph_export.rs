use crate::relation::graph::RelationGraph;
use petgraph::dot::Config;
use std::collections::HashMap;
use std::fmt::Error;
use std::fs::File;
use std::io::Write;

/// export API
impl RelationGraph {
    fn to_simple(&self) -> RelationGraph {
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
        return graph;
    }
    pub fn export_dot(&self, file_path: &str) {
        let graph = self.to_simple();

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
