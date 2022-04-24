use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct PendingTransactionReq {
    pub txn: String,
}

pub struct NodePool {
    pub nodes: Vec<Node>,
}

impl NodePool {
    pub fn new(node_addrs: Vec<SocketAddr>) -> NodePool {
        let nodes: Vec<Node> = node_addrs.into_iter().map(|n| Node::new(n)).collect();
        NodePool { nodes }
    }
}

pub struct Node {
    pub addr: SocketAddr,
}

impl Node {
    pub fn new(addr: SocketAddr) -> Node {
        Node { addr }
    }
}
