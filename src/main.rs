mod blockchain;
mod clients;
mod config;
mod server;

#[tokio::main]
async fn main() {
    let config = config::init();
    server::start(config).await;
}
