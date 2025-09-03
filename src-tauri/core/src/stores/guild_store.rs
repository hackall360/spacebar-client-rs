use super::store::Store;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GuildStoreState {
    // TODO: replicate fields from TypeScript GuildStore.ts
}

#[derive(Debug, Clone)]
pub enum GuildStoreEvent {
    Updated,
}

pub type GuildStore = Store<GuildStoreState, GuildStoreEvent>;
