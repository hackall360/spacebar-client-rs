use super::store::Store;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PresenceStoreState {
    // TODO: replicate fields from TypeScript PresenceStore.ts
}

#[derive(Debug, Clone)]
pub enum PresenceStoreEvent {
    Updated,
}

pub type PresenceStore = Store<PresenceStoreState, PresenceStoreEvent>;
