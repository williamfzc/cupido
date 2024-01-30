use crate::relation::graph::GraphSize;
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
                .route("/-/commits", get(file_related_commit_handler))
                .route("/-/issues", get(file_related_issue_handler)),
        )
        .route("/size", get(size_handler))
        .route("/", get(root_handler));
}

async fn size_handler() -> axum::Json<GraphSize> {
    let conf = crate::server::app::SERVER_CONFIG.read().unwrap();
    axum::Json(conf.graph.size())
}

async fn root_handler() -> axum::Json<Desc> {
    axum::Json(Desc {
        version: crate::server::app::VERSION.to_string(),
    })
}

async fn file_related_commit_handler(Query(params): Query<Params>) -> axum::Json<Vec<String>> {
    let conf = crate::server::app::SERVER_CONFIG.read().unwrap();
    return match conf.graph.related_commits(&params.file) {
        Ok(commits) => axum::Json(commits),
        Err(error) => {
            error!("file_related_commit error: {}", error);
            axum::Json(Vec::new())
        }
    };
}

async fn file_related_issue_handler(Query(params): Query<Params>) -> axum::Json<Vec<String>> {
    let conf = crate::server::app::SERVER_CONFIG.read().unwrap();
    return match conf.graph.related_issues(&params.file) {
        Ok(issues) => axum::Json(issues),
        Err(error) => {
            error!("file_related_issues error: {}", error);
            axum::Json(Vec::new())
        }
    };
}

#[derive(Debug, Deserialize)]
struct Params {
    file: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Desc {
    version: String,
}
