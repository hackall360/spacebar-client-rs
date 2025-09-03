use super::store::Store;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageStoreState {
    // TODO: replicate fields from TypeScript MessageStore.ts
}

#[derive(Debug, Clone)]
pub enum MessageStoreEvent {
    Updated,
}

pub type MessageStore = Store<MessageStoreState, MessageStoreEvent>;
