use crate::blockchain::Node;
use anyhow::Result;
use serde_json::Value;

#[jsonrpc_client::api]
pub trait BlockchainNode {
    async fn pending_transaction_submit(&self, txn: String) -> Value;
    async fn pending_transaction_status(&self, hash: String) -> Value;
}

#[jsonrpc_client::implement(BlockchainNode)]
pub struct BlockchainNodeClient {
    inner: reqwest::Client,
    base_url: reqwest::Url,
}

impl BlockchainNodeClient {
    pub fn new_from_node(node: &Node) -> Result<BlockchainNodeClient> {
        let addr = format!("http://{}/", node.addr.to_string());
        Ok(BlockchainNodeClient {
            inner: reqwest::Client::new(),
            base_url: addr.parse()?,
        })
    }
}
