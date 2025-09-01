use super::store::Store;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GatewayConnectionStoreState {
    // TODO: replicate fields from TypeScript GatewayConnectionStore.ts
}

#[derive(Debug, Clone)]
pub enum GatewayConnectionStoreEvent {
    Updated,
}

pub type GatewayConnectionStore = Store<GatewayConnectionStoreState, GatewayConnectionStoreEvent>;
