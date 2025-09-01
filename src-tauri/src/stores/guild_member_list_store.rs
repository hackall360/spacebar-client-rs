use super::store::Store;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GuildMemberListStoreState {
    // TODO: replicate fields from TypeScript GuildMemberListStore.ts
}

#[derive(Debug, Clone)]
pub enum GuildMemberListStoreEvent {
    Updated,
}

pub type GuildMemberListStore = Store<GuildMemberListStoreState, GuildMemberListStoreEvent>;
