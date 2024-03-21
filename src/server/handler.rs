use crate::collector::config::Config;
use crate::relation::graph::GraphSize;
use crate::server::handler_ext::{
    author_related_commits_handler, authors, commit_related_authors_handler,
};
use axum::extract::Query;
use axum::routing::get;
use axum::Router;
use serde_derive::{Deserialize, Serialize};
use tracing::error;

pub fn create_router() -> Router {
    return Router::new()
        .nest(
            "/file",
            Router::new()
                .route("/-/commits", get(file_related_commits_handler))
                .route("/-/issues", get(file_related_issues_handler)),
        )
        .nest(
            "/issue",
            Router::new()
                .route("/-/files", get(issue_related_files_handler))
                .route("/-/commits", get(issue_related_commits_handler))
                .route("/list", get(issues)),
        )
        .nest(
            "/commit",
            Router::new()
                .route("/-/files", get(commit_related_files_handler))
                .route("/-/issues", get(commit_related_issues_handler)),
        )
        .route("/size", get(size_handler))
        .route("/", get(root_handler))
        // extensions
        .nest(
            "/author",
            Router::new()
                .route("/-/commits", get(author_related_commits_handler))
                .route("/list", get(authors)),
        )
        .nest(
            "/commit",
            Router::new().route("/-/authors", get(commit_related_authors_handler)),
        );
}

async fn size_handler() -> axum::Json<GraphSize> {
    let conf = crate::server::app::SERVER_CONFIG.read().unwrap();
    axum::Json(conf.graph.size())
}

async fn root_handler() -> axum::Json<Desc> {
    let conf = crate::server::app::SERVER_CONFIG.read().unwrap();
    axum::Json(Desc {
        version: crate::server::app::VERSION.to_string(),
        graph_conf: conf.graph.conf.clone(),
    })
}

async fn file_related_commits_handler(Query(params): Query<FileParams>) -> axum::Json<Vec<String>> {
    let conf = crate::server::app::SERVER_CONFIG.read().unwrap();
    return match conf.graph.file_related_commits(&params.file) {
        Ok(commits) => axum::Json(commits),
        Err(error) => {
            error!("file_related_commit error: {}", error);
            axum::Json(Vec::new())
        }
    };
}

async fn file_related_issues_handler(Query(params): Query<FileParams>) -> axum::Json<Vec<String>> {
    let conf = crate::server::app::SERVER_CONFIG.read().unwrap();
    return match conf.graph.file_related_issues(&params.file) {
        Ok(issues) => axum::Json(issues),
        Err(error) => {
            error!("file_related_issues error: {}", error);
            axum::Json(Vec::new())
        }
    };
}

async fn issue_related_files_handler(Query(params): Query<IssueParams>) -> axum::Json<Vec<String>> {
    let conf = crate::server::app::SERVER_CONFIG.read().unwrap();
    return match conf.graph.issue_related_files(&params.issue) {
        Ok(issues) => axum::Json(issues),
        Err(error) => {
            error!("file_related_issues error: {}", error);
            axum::Json(Vec::new())
        }
    };
}

async fn issue_related_commits_handler(
    Query(params): Query<IssueParams>,
) -> axum::Json<Vec<String>> {
    let conf = crate::server::app::SERVER_CONFIG.read().unwrap();
    return match conf.graph.issue_related_commits(&params.issue) {
        Ok(issues) => axum::Json(issues),
        Err(error) => {
            error!("file_related_issues error: {}", error);
            axum::Json(Vec::new())
        }
    };
}

async fn issues() -> axum::Json<Vec<String>> {
    let conf = crate::server::app::SERVER_CONFIG.read().unwrap();
    return axum::Json(conf.graph.issues());
}

async fn commit_related_files_handler(
    Query(params): Query<CommitParams>,
) -> axum::Json<Vec<String>> {
    let conf = crate::server::app::SERVER_CONFIG.read().unwrap();
    return match conf.graph.commit_related_files(&params.commit) {
        Ok(issues) => axum::Json(issues),
        Err(error) => {
            error!("file_related_issues error: {}", error);
            axum::Json(Vec::new())
        }
    };
}

async fn commit_related_issues_handler(
    Query(params): Query<CommitParams>,
) -> axum::Json<Vec<String>> {
    let conf = crate::server::app::SERVER_CONFIG.read().unwrap();
    return match conf.graph.commit_related_issues(&params.commit) {
        Ok(issues) => axum::Json(issues),
        Err(error) => {
            error!("file_related_issues error: {}", error);
            axum::Json(Vec::new())
        }
    };
}

#[derive(Debug, Deserialize)]
struct FileParams {
    file: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CommitParams {
    pub(crate) commit: String,
}

#[derive(Debug, Deserialize)]
struct IssueParams {
    issue: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Desc {
    version: String,
    graph_conf: Config,
}
