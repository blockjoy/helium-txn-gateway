use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

pub struct Config {
    pub server_addr: SocketAddr,
    pub node_addrs: Vec<SocketAddr>,
    pub log_level: String,
}

pub fn init() -> Config {
    dotenv().ok();

    let server_addr: SocketAddr = env::var("SERVER_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:3000".into())
        .parse()
        .expect("To have a valid SOCKET_ADDR");

    let node_addrs: Vec<SocketAddr> = env::var("NODES")
        .unwrap_or_else(|_| "".into())
        .split(",")
        .map(|i| i.trim().parse().expect("Valid node addr."))
        .collect();

    let log_level = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "helium_txn_gateway=debug,tower_http=debug".into());

    Config {
        server_addr,
        node_addrs,
        log_level,
    }
}
