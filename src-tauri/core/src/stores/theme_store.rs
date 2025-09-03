use super::store::Store;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeStoreState {
    // TODO: replicate fields from TypeScript ThemeStore.ts
}

#[derive(Debug, Clone)]
pub enum ThemeStoreEvent {
    Updated,
}

pub type ThemeStore = Store<ThemeStoreState, ThemeStoreEvent>;
