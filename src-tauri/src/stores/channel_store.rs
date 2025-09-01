use super::store::Store;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChannelStoreState {
    // TODO: replicate fields from TypeScript ChannelStore.ts
}

#[derive(Debug, Clone)]
pub enum ChannelStoreEvent {
    Updated,
}

pub type ChannelStore = Store<ChannelStoreState, ChannelStoreEvent>;
