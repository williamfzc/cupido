use crate::collector::config::Config as CollectorConfig;
use petgraph::graph::{NodeIndex, UnGraph};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(PartialEq, Eq)]
pub(crate) enum NodeType {
    File(Option<FileData>),
    Commit(Option<CommitData>),
    Issue(Option<IssueData>),
    Author(Option<AuthorData>),
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) struct FileData {}

#[derive(PartialEq, Eq, Debug)]
pub(crate) struct CommitData {}

#[derive(PartialEq, Eq, Debug)]
pub(crate) struct IssueData {}

#[derive(PartialEq, Eq, Debug)]
pub(crate) struct AuthorData {}

#[derive(Debug)]
pub(crate) enum EdgeType {
    // core
    File2Commit,
    File2Issue,
    Commit2Issue,

    // options
    Author2Commit,
}

impl Display for EdgeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Eq)]
pub struct NodeData {
    pub(crate) name: Arc<String>,
    pub(crate) _node_type: NodeType,
}

impl Display for NodeData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.name)
    }
}

pub(crate) type NodeMapping = HashMap<Arc<String>, NodeIndex>;

pub struct RelationGraph {
    pub(crate) file_mapping: NodeMapping,
    pub(crate) commit_mapping: NodeMapping,
    pub(crate) issue_mapping: NodeMapping,
    pub(crate) author_mapping: NodeMapping,
    pub(crate) g: UnGraph<NodeData, EdgeType>,
    pub(crate) conf: CollectorConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GraphSize {
    pub(crate) file_size: usize,
    pub(crate) commit_size: usize,
    pub(crate) issue_size: usize,
}
