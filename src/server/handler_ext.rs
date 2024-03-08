use crate::server::handler::CommitParams;
use axum::extract::Query;
use serde_derive::Deserialize;
use tracing::error;

#[derive(Debug, Deserialize)]
pub(crate) struct AuthorParams {
    author: String,
}

pub(crate) async fn author_related_commits_handler(
    Query(params): Query<AuthorParams>,
) -> axum::Json<Vec<String>> {
    let conf = crate::server::app::SERVER_CONFIG.read().unwrap();
    return match conf.graph.author_related_commits(&params.author) {
        Ok(issues) => axum::Json(issues),
        Err(error) => {
            error!("author_related_commits error: {}", error);
            axum::Json(Vec::new())
        }
    };
}

pub(crate) async fn commit_related_authors_handler(
    Query(params): Query<CommitParams>,
) -> axum::Json<Vec<String>> {
    let conf = crate::server::app::SERVER_CONFIG.read().unwrap();
    return match conf.graph.commit_related_authors(&params.commit) {
        Ok(authors) => axum::Json(authors),
        Err(error) => {
            error!("commit_related_authors error: {}", error);
            axum::Json(Vec::new())
        }
    };
}

pub(crate) async fn authors() -> axum::Json<Vec<String>> {
    let conf = crate::server::app::SERVER_CONFIG.read().unwrap();
    return match conf.graph.authors() {
        Ok(authors) => axum::Json(authors),
        Err(error) => {
            error!("authors error: {}", error);
            axum::Json(Vec::new())
        }
    };
}
