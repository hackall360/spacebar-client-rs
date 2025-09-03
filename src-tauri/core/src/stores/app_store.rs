use super::store::Store;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppStoreState {
    // TODO: replicate fields from TypeScript AppStore.ts
}

#[derive(Debug, Clone)]
pub enum AppStoreEvent {
    Updated,
}

pub type AppStore = Store<AppStoreState, AppStoreEvent>;
