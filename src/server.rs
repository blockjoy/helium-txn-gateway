use crate::{blockchain, config::Config};
use axum::{
    extract::{Extension, Json, Path},
    routing::{get, post},
    Router,
};

use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn start(config: Config) {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(config.log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let node_pool: NodePool = NodePool::new(blockchain::NodePool::new(config.node_addrs));

    let app = Router::new()
        .route(
            "/v1/pending_transactions/:hash",
            get(get_pending_transaction),
        )
        .route("/v1/pending_transactions", post(create_pending_transaction))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(Extension(node_pool))
                .into_inner(),
        );

    tracing::debug!("listening on {}", config.server_addr);
    axum::Server::bind(&config.server_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

type NodePool = Arc<blockchain::NodePool>;

async fn create_pending_transaction(
    Json(payload): Json<blockchain::PendingTransactionReq>,
    Extension(state): Extension<NodePool>,
) {
    unimplemented!()
}

async fn get_pending_transaction(Path(hash): Path<String>, Extension(state): Extension<NodePool>) {
    unimplemented!()
}
