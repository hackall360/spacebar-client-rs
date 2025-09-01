use super::store::Store;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RoleStoreState {
    // TODO: replicate fields from TypeScript RoleStore.ts
}

#[derive(Debug, Clone)]
pub enum RoleStoreEvent {
    Updated,
}

pub type RoleStore = Store<RoleStoreState, RoleStoreEvent>;
