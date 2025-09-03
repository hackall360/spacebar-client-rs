use super::store::Store;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GuildMemberStoreState {
    // TODO: replicate fields from TypeScript GuildMemberStore.ts
}

#[derive(Debug, Clone)]
pub enum GuildMemberStoreEvent {
    Updated,
}

pub type GuildMemberStore = Store<GuildMemberStoreState, GuildMemberStoreEvent>;
