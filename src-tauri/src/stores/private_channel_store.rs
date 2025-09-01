use super::store::Store;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrivateChannelStoreState {
    // TODO: replicate fields from TypeScript PrivateChannelStore.ts
}

#[derive(Debug, Clone)]
pub enum PrivateChannelStoreEvent {
    Updated,
}

pub type PrivateChannelStore = Store<PrivateChannelStoreState, PrivateChannelStoreEvent>;
