use crate::{blockchain, config::Config};
use axum::{
    extract::{Extension, Json, Path},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::signal::unix::{signal, SignalKind};
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
        .route("/health", get(health_check))
        .route(
            "/v1/pending_transactions/:hash",
            get(get_pending_transaction_status),
        )
        .route("/v1/pending_transactions", post(create_pending_transaction))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(Extension(node_pool))
                .into_inner(),
        );

    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

    let signal_handler = tokio::spawn(async {
        tokio::pin! {
          let interrupt = signal(SignalKind::interrupt()).expect("could not open SIGINT channel");
          let quit = signal(SignalKind::quit()).expect("could not open SIGQUIT channel");
          let term = signal(SignalKind::terminate()).expect("could not open SIGTERM channel");
        };

        loop {
            tokio::select! {
              _ = (&mut interrupt).recv() => {
                  tracing::info!("SIGINT received");
                  break;
              }
              _ = (&mut quit).recv() => {
                tracing::info!("SIGQUIT received");
                  break;
              }
              _ = (&mut term).recv() => {
                tracing::info!("SIGTERM received");
                  break;
              }
            }
        }

        shutdown_tx
            .send(())
            .expect("could not send shutdown signal");
    });
    tracing::info!("Waiting for SIGTERM/SIGQUIT for graceful shutdown");

    tracing::info!("listening on {}", config.server_addr);
    axum::Server::bind(&config.server_addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            shutdown_rx.await.ok();
        })
        .await
        .expect("could not start HTTP Server.");

    signal_handler
        .await
        .expect("error with shutdown handler task");
}

type NodePool = Arc<blockchain::NodePool>;

/// Creates a pending transaction
async fn create_pending_transaction(
    Json(payload): Json<blockchain::PendingTransactionReq>,
    Extension(state): Extension<NodePool>,
) {
    unimplemented!()
}

// Looks up a pending transaction status
async fn get_pending_transaction_status(
    Path(hash): Path<String>,
    Extension(state): Extension<NodePool>,
) {
    unimplemented!()
}

//Health Check
async fn health_check() -> &'static str {
    "OK"
}
