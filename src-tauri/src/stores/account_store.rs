use super::store::Store;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountStoreState {
    // TODO: replicate fields from TypeScript AccountStore.ts
}

#[derive(Debug, Clone)]
pub enum AccountStoreEvent {
    Updated,
}

pub type AccountStore = Store<AccountStoreState, AccountStoreEvent>;
