use super::store::Store;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReadStateStoreState {
    // TODO: replicate fields from TypeScript ReadStateStore.ts
}

#[derive(Debug, Clone)]
pub enum ReadStateStoreEvent {
    Updated,
}

pub type ReadStateStore = Store<ReadStateStoreState, ReadStateStoreEvent>;
