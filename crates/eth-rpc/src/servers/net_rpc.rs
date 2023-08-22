use jsonrpsee::core::{async_trait, RpcResult as Result};
use kakarot_rpc_core::client::constants::CHAIN_ID;
use reth_network_api::PeersInfo;
use reth_primitives::U64;
use reth_rpc_types::PeerCount;
use crate::api::net_api::NetApiServer;

/// The RPC module for the implementing Net api
// #[derive(Default)]
pub struct NetRpc<P> {
    // pub network: Arc<dyn PeersInfo>, 
    network: P
}

impl<P: PeersInfo> NetRpc<P> {
    pub fn new(network: P) -> Self {
        Self { network}
    }
}

#[async_trait]
impl<P: PeersInfo + 'static> NetApiServer for NetRpc<P> {
    /// Get the protocol version of the Kakarot Starknet RPC.
    fn version(&self) -> Result<U64> {
        // let protocol_version = 1_u64;
        // Ok(protocol_version.into())
        Ok(U64::from(CHAIN_ID))
    }

    fn peer_count(&self) -> Result<PeerCount> {
        Ok(PeerCount::Hex(self.network.num_connected_peers().into()))
    }

    fn listening(&self) -> Result<bool> {
        Ok(true)
    }
}

