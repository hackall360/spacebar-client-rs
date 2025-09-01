use super::store::Store;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserStoreState {
    // TODO: replicate fields from TypeScript UserStore.ts
}

#[derive(Debug, Clone)]
pub enum UserStoreEvent {
    Updated,
}

pub type UserStore = Store<UserStoreState, UserStoreEvent>;
