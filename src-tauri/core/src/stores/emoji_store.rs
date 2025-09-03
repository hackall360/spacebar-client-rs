use super::store::Store;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmojiStoreState {
    // TODO: replicate fields from TypeScript EmojiStore.ts
}

#[derive(Debug, Clone)]
pub enum EmojiStoreEvent {
    Updated,
}

pub type EmojiStore = Store<EmojiStoreState, EmojiStoreEvent>;
