use super::store::Store;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExperimentsStoreState {
    // TODO: replicate fields from TypeScript ExperimentsStore.ts
}

#[derive(Debug, Clone)]
pub enum ExperimentsStoreEvent {
    Updated,
}

pub type ExperimentsStore = Store<ExperimentsStoreState, ExperimentsStoreEvent>;
