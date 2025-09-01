use super::store::Store;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdaterStoreState {
    // TODO: replicate fields from TypeScript UpdaterStore.ts
}

#[derive(Debug, Clone)]
pub enum UpdaterStoreEvent {
    Updated,
}

pub type UpdaterStore = Store<UpdaterStoreState, UpdaterStoreEvent>;
